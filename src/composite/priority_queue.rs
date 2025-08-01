/*! An adaptable priority queue implementation

# About
This structure adds key and priority mutation as well as entry removal operations over the base [binary heap](crate::trees::bin_heap) in this library. You can use either structure to implement Dijkstra's algorithm, but this adaptable priority queue implementation avoids creating duplicate key entries which reduces spatial complexity and can _potentially_ improve temporal performance.

The primary struct [PriorityQueue<K, P>](crate::composite::priority_queue::PriorityQueue) provides the ability to mutate both key `K` and priority `P` after insertion while maintaining _O(log(n))_ (or better) operations. It is the caller's responsibility to ensure that the keying schemes they use guarantees uniqueness to maintain the structure's integrity. 

You have the option to YOLO your way through your list by calling `put(K, P)` which overwrites the priorty for existing keys, or by using `try_put(K, P)` which returns [Result] if the key already exists in the queue.

# Design
The composite desgin of this structure provides _O(log(n))_ insert, remove, and key/priority mutation operations. The structure uses a `Vec`-based [heap](crate::trees::bin_heap) as its primary backing and ordering structure. In addition to the heap, this design also uses a [map](crate::maps::probing_hash_table) for (private) `find(k)` opertions that run in _O(1)_ time. The map lookups allow heap mutation operaitons to avoid _O(n)_ (_n/2_ average) lookups. Due to the map's deletion logic, removed items are technically leaked which grows the map with key mutations. Key mutation operations amortize temporal complexity to _O(1)_. 

This structure uses Rust's [default hasher](std::hash::DefaultHasher) to hash keys. The default hasher (currently) represents a cryptographically strong [SipHash-1-3](https://en.wikipedia.org/wiki/SipHash) implementation, though the implementation is subject to change. In practice, hashing small key objects <= 128 bytes provide similar performance as ULID generation, but keep in mind that hashing arbitrarily large key objects may incur performance penalties.

# Example

```rust

```

*/

use crate::maps::probing_hash_table::ProbingHashTable;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::result::Result::Err;

#[derive(Clone, Debug)]
struct Entry<K, P> {
    key: K,
    priority: P,
}

/// # About
///
/// See the [module-level documentation](crate::composite::priority_queue) for more information.
#[derive(Debug)]
pub struct PriorityQueue<K, P> 
where 
    K: Clone + Hash,
    P: Clone + Ord + PartialEq,
{
    // Sorted backing structure of key:priority pairs
    heap: Vec<Entry<K, P>>,
    // Tracks entry indexes in the heap as <hashed_K, index>
    map: ProbingHashTable<usize, usize>, 
}
impl<K, P> Default for PriorityQueue<K, P>
where
    K: std::fmt::Debug + Clone + Hash,
    P: Clone + std::fmt::Debug + Ord + PartialEq,
 {
    fn default() -> Self {
        Self::new()
    }
}
impl<K, P> PriorityQueue<K, P> 
where 
    K: std::fmt::Debug + Clone + Hash,
    P: Clone + std::fmt::Debug + Ord + PartialEq,
{
    // PUBLIC API
    /////////////

    /// Creates a new, zero-sized `PriorityQueue`.
    /// Warning: Unimplemented
    pub fn new() -> Self {
        PriorityQueue {
            heap: Vec::new(),
            map: ProbingHashTable::new(),
        }
    }

    /// Creates a new `PriorityQueue` that pre-allocates to the given capacity.
    pub fn new_with_capacity(capacity: usize) -> Self {
        PriorityQueue {
            heap: Vec::with_capacity(capacity),
            map: ProbingHashTable::new(),
        }
    }

    /// Returns the number of elements in the heap.
    pub fn size(&self) -> usize {
        self.heap.len()
    }

    /// Returns `true` if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty() && self.map.is_empty()
    }

    /// Returns `true` if the provided key exists within the priority queue.
    pub fn contains(&self, key: K) -> bool {
        let hash = Self::hash(&key);
        self.map.contains(hash)
    }

    /// Adds an element to the heap in _O(log(n))_ time. It is the 
    /// caller's responsibility to ensure key uniqueness; this function 
    /// overwrites priority values for duplicate keys.
    pub fn put(&mut self, key: K, priority: P) {
        
        let hash = Self::hash(&key);

        // Add the entry to the backing structure
        self.heap.push(Entry { key, priority });

        // Sift the entry up the heap, tracking index value
        let index = self.sift_up(self.heap.len() - 1);

        // Update the map with a unique key and the element's index value
        self.map.put(hash, index); 
    }

    /// Attempts to add an element to the heap in _O(log(n))_ time.
    /// Returns an error if the key already exists in the queue.
    pub fn try_put(&mut self, key: K, priority: P) -> Result<(), &str> {
        
        let hash = Self::hash(&key);

        // Check to see if the structure already contains the element
        // and if it does not, add it
        if self.map.contains(hash) {
            Err("Error: Duplicate key detected")
        } else {

            // Add the entry to the backing structure
            self.heap.push(Entry { key, priority });

            // Sift the entry up the heap, tracking index value
            let index = self.sift_up(self.heap.len() - 1);

            // Update the map with a unique key and the element's index value
            self.map.put(hash, index); 

            Ok(())
        }
    }

    /// Returns a tuple containing an immutable reference to the data 
    /// at the top of the front of the queue in _O(1)_ time, if the 
    /// queue is not empty. Otherwise, returns `None`.
    pub fn peek(&self) -> Option<(&K, &P)> {
        if self.heap.is_empty() {
            None
        } else {
            Some((&self.heap[0].key, &self.heap[0].priority))
        }
    }

    /// Operates like `put(K, P)` but returns a `Result` that contains
    /// the old priority, if it exists. Operation requires _O(log(n))_ time.
    pub fn mutate_priority(&mut self, key: K, new_priority: P) -> Result<P, &str> {
        let hashed_key = Self::hash(&key);
    
        // If the key is in the system, update its priority
        if let Some(index) = self.map.get(hashed_key).cloned() {

            let entry = &mut self.heap[index];
            
            // Bind the old priority and update the entry
            let old_priority = entry.priority.clone();
            entry.priority = new_priority.clone();
    
            // Re-heapify from current index
            if new_priority < old_priority {
                self.sift_up(index);
            } else {
                self.sift_down(index);
            }
    
            Ok(old_priority)
        } else {
            Err("Error: Key does not exist in the queue")
        }
    }

    /// Attempts to replace the key in a key:value pair in the queue in _O(1)_ time, 
    /// if it exists. If the key does not exist, the operation returns an error.
    pub fn mutate_key(&mut self, old_key: K, new_key: K) -> Result<K, &str> {
        let hashed_old_key = Self::hash(&old_key);
        let hashed_new_key = Self::hash(&new_key);
    
        // Ensure the old key exists
        if let Some(&index) = self.map.get(hashed_old_key) {
            // Prevent overwriting an existing key
            if self.map.contains(hashed_new_key) {
                return Err("Error: New key already exists in the queue");
            }
    
            // Remove old key from the map
            self.map.remove(hashed_old_key);
    
            // Insert new key pointing to same index
            self.map.put(hashed_new_key, index);
    
            // Update the heap entry
            self.heap[index].key = new_key;
    
            Ok(old_key)
        } else {
            Err("Error: Old key does not exist in the queue")
        }
    }

    /// Returns and removes the element at the top of the heap in _O(log(n))_ time.
    pub fn pop(&mut self) -> Option<(K, P)> {
        if !self.heap.is_empty() {
            let node = self.heap.swap_remove(0); // O(1)
            self.sift_down(0); // O(log(n))
            Some((node.key, node.priority))
        } else { None }
    }

    /// Removes and returns the key:priority pair for the given key in _O(log(n))_ time.
    pub fn remove(&mut self, key: K) -> Option<(K, P)> {
        let hashed_key = Self::hash(&key);
    
        // If the key is in the queue, remove it
        if let Some(&index) = self.map.get(hashed_key) {
            self.map.remove(hashed_key);
    
            // Special case: last element 
            if index == self.heap.len() - 1 {
                let removed_entry = self.heap.pop().unwrap();
                return Some((removed_entry.key, removed_entry.priority));
            }
    
            // swap_remove allows you to maintain O(log(n)) removals
            // by avoiding O(n) shifts with remove
            let removed_entry = self.heap.swap_remove(index);
    
            // Update map for swapped item
            let moved_key = self.heap[index].key.clone();
            self.map.put(Self::hash(&moved_key), index);
    
            // Decide whether to sift up or down to restore heap
            let parent_index = (index - 1) / 2;
            if index > 0 && self.heap[index].priority < self.heap[parent_index].priority {
                self.sift_up(index);
            } else {
                self.sift_down(index);
            }
    
            Some((removed_entry.key, removed_entry.priority))
        } else {
            None // No op; the key isn't in the queu
        }
    }

    // UTILITIES
    ////////////

    /// Utility function for hashing keys
    fn hash(key: &K) -> usize {
        // std::hash::random::DefaultHasher
        let mut hasher = DefaultHasher::new();
        // core::hash::Hash::hash()
        key.hash(&mut hasher);
        // std::hash::random::DefaultHasher::finish()
        hasher.finish() as usize
    }

    /// Sifts a new node up the heap towards the root to maintain the heap property
    fn sift_up(&mut self, mut index: usize) -> usize {
        while index > 0 {
            let parent_index = (index - 1) / 2;
    
            if self.heap[index].priority < self.heap[parent_index].priority {
                // Bind the keys
                let key_child = self.heap[index].key.clone();
                let key_parent = self.heap[parent_index].key.clone();
    
                // Swap elements in the heap
                self.heap.swap(index, parent_index);
    
                // Update map after the swap
                self.map.put(Self::hash(&key_child), parent_index);
                self.map.put(Self::hash(&key_parent), index);
                
                // Move to the parent's old position to continue sifting
                index = parent_index;
            } else {
                break;
            }
        }
        index
    }

    /// Sifts a new node downward in the tree to maintain the heap property
    fn sift_down(&mut self, mut index: usize) -> usize {
        loop {
            let left_child = 2 * index + 1;
            let right_child = 2 * index + 2;
            let mut target = index; 

            // 1) Finds a target index to swap. Sibling order is not 
            // guaranteed, but you still need to check both children to 
            // ensure heap property when sifting
            //
            // NOTE: (Adjust < to > for a max heap)
            if left_child < self.heap.len()
                && self.heap[left_child].priority < self.heap[target].priority {
                target = left_child;
            }
            if right_child < self.heap.len()
                && self.heap[right_child].priority < self.heap[target].priority { 
                target = right_child;
            }
            
            // 2) If the target is not equal to the index, do the swap operation 
            // and continue sifting, otherwise the heap property is true
            if target != index {

                // Bind the keys
                let key_child = self.heap[index].key.clone();
                let key_parent = self.heap[target].key.clone();
    
                // Swap elements in the heap
                self.heap.swap(index, target);
    
                // Update map after the swap
                self.map.put(Self::hash(&key_child), target);
                self.map.put(Self::hash(&key_parent), index);

                index = target; // Move down
            } else {
                break; // No-op, heap is heapy 🍃🧘
            }
        };
        index
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    /// This test uses a mock scenario of a trip booking system
    /// where each key is a name of a person and each person
    /// can book either economy, business, or first class ticket
    #[test]
    fn basic() {

        let mut queue: PriorityQueue<&str, usize> = PriorityQueue::new();

        // Initial list of passengers and their boarding groups
        let view = [
            ("Dichael", 3),
            ("Sleve", 2),
            ("Cork", 3),
            ("Damiel", 2),
            ("Peter", 1),
            ("Bobson", 3), 
            ("Flank", 3)
        ];

        // Populate the queue
        for e in view.iter() {
            queue.put(e.0, e.1)
        }

        // DEBUG PRINT: initial state
        eprintln!("Pre-mutations: {:#?}", queue);

        // Attempt to place duplicate key:value pair
        assert!(queue.try_put("Dichael", 2).is_err());

        // Attempt to mutate priority
        assert!(queue.mutate_priority("Cork", 1).is_ok()); // increase
        assert!(queue.mutate_priority("Damiel", 3).is_ok()); // decrease

        // Attempt to mutate key
        assert!(queue.mutate_key("Peter", "The Peter").is_ok());
        
        // Remove passengers
        assert_eq!(queue.remove("Sleve"), Some(("Sleve", 2)));
        assert_eq!(queue.remove("Flank"), Some(("Flank", 3)));

        // Testing updated membership
        assert!(queue.contains("The Peter"));
        assert!(queue.contains("Damiel"));
        assert!(queue.contains("Cork"));
        assert!(queue.contains("Dichael"));
        assert!(queue.contains("Bobson"));
        assert!(!queue.contains("Sleve"));
        assert!(!queue.contains("Flank"));

        // Checks to make sure the final composition is correct.
        // The pop operation does not guarantee ordering of
        // equal priorities.
        let mut passengers: Vec<(&str, usize)> = Vec::new();
        for _ in 0..queue.heap.len() {
            passengers.push(queue.pop().unwrap());
        }

        // DEBUG PRINT: proper priority ordering
        eprintln!("Queue popped to list to illustrate priority ordering:\n\t{:?}", passengers);

        // Tests to ensure updates took
        assert!(passengers.contains(&("The Peter", 1)));
        assert!(passengers.contains(&("Damiel", 3)));
        assert!(passengers.contains(&("Cork", 1)));
        assert!(passengers.contains(&("Dichael", 3)));

        // Just to make sure we're not fibbin
        assert!(!passengers.contains(&("Dichael", 2)));
        assert!(!passengers.contains(&("Cork", 3)));
        assert!(!passengers.contains(&("Damiel", 2)));
        assert!(!passengers.contains(&("Peter", 1)));
        assert!(!passengers.contains(&("Sleve", 2)));
        assert!(!passengers.contains(&("Flank", 3)));

        // Triggers failure for debug print
        //assert!(passengers.contains(&("TEST", 69)));
    }
}
