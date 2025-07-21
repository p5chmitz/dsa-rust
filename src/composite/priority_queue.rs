/*! An adaptable priority queue implementation

# About
This structure uses an arena-allocated (`Vec`-based) heap as its primary backing structure which provides `O(log(n))` insert, remove, and key/value mutation operations. The structure also uses an index lookup table for `O(1)` index lookups within the heap.

This structure uses keys as priority indicators. Entries with identical keys (i.e., equal priorities) are ordered by chronological insertion using FIFO logic. For example, inserting the key-value pairs (3, "Peter") followed by (3, "Dichael") ensures that "Peter" is popped before "Dichael".

# Design

# Example

```rust
```

*/

use crate::maps::probing_hash_table::ProbingHashTable;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct Node<K, V> 
where K: Ord + Hash {
    key: K,   // Priority of the entry; must be orderable and hashable for indexing and reordering
    value: V, // Associated payload; the actual data stored in the queue
}

/// # About
///
/// See the [module-level documentation](crate::composite::priority_queue) for more information.
#[derive(Debug)]
pub struct PriorityQueue<K, V> 
where 
    K: Clone + Ord + Hash,
{
    heap: Vec<Node<K, V>>, // Sorted backing structure
    map: ProbingHashTable<K, usize>, // Tracks entry indexes in the heap
}
impl<K, V> PriorityQueue<K, V> 
where 
    K: Ord + Hash + std::fmt::Debug + Clone
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

    /// Adds an element to the heap in `O(log(n))` time.
    pub fn insert(&mut self, key: K, value: V) {
        // 1) Create and insert a Node into the array in a way that maintains the
        // complete binary tree structure
        let entry = Node {
            key,
            value
        };
        self.heap.push(entry);

        // 2) Sift up, tracking index value
        self.sift_up(self.heap.len() - 1);
        
        // 3) Update the map

    }

    /// Returns `true` if the provided key exists within the priority queue.
    /// Warning: Unimplemented
    pub fn contains(&self, key: K) -> bool {
        true
    }

    /// Returns an immutable reference to the data at the top of the heap 
    /// in `O(1)` time.
    pub fn peek(&self) -> Option<(&K, &V)> {
        if self.heap.is_empty() {
            None
        } else {
            Some((&self.heap[0].key, &self.heap[0].value))
        }
    }

    pub fn mutate_priority(&mut self, old_key: K, new_key: K) {}

    /// Returns and removes the element at the top of the heap in `O(log(n))` time.
    pub fn pop(&mut self) -> Option<(K, V)> {
        if !self.heap.is_empty() {
            let node = self.heap.swap_remove(0); // O(1)
            self.sift_down(0); // O(log(n))
            Some((node.key, node.value))
        } else { None }
    }

    // UTILITIES
    ////////////

    /// Sifts a node upward from the given index in the tree to maintain the 
    /// heap property.
    fn sift_up(&mut self, mut index: usize) {
        // Only sift up if the element is not the root (index 0)
        while index > 0 {
            // Calculate the parent's index
            let parent_index = (index - 1) / 2;
    
            // For a Min-Heap: If the current node's priority is LESS than its 
            // parent's, swap.
            // For a Max-Heap: If the current node's priority is GREATER than 
            // its parent's, swap.
            // Switch < to > for max heap
            if self.heap[index].key < self.heap[parent_index].key {
                self.heap.swap(index, parent_index);
                // Move up to the parent's old position to continue sifting
                index = parent_index; 
            } else {
                break; // No-op, heap is heapy 🍃🧘
            }
        }
    }

    /// Sifts a new node downward in the tree to maintain the heap property.
    fn sift_down(&mut self, mut index: usize) {
        loop {
            let left_child = 2 * index + 1;
            let right_child = 2 * index + 2;
            let mut target = index; 

            // 1) Finds a target index to swap. Sibling order is not guaranteed, 
            // but you still need to check both children to ensure heap property 
            // when sifting
            //
            // NOTE: (Adjust < to > for a max heap)
            if left_child < self.heap.len()
                && self.heap[left_child].key < self.heap[target].key {
                target = left_child;
            }
            if right_child < self.heap.len()
                && self.heap[right_child].key < self.heap[target].key { 
                target = right_child;
            }
            
            // 2) If the target is not equal to the index, do the swap operation 
            // and continue sifting, otherwise the heap property is true
            if target != index {
                self.heap.swap(index, target);
                index = target; // Move down
            } else {
                break; // No-op, heap is heapy 🍃🧘
            }
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    //#[test]
    // Tests two approaches to heap sort:
    // 1) Creating a separate binary heap buffer, pushing values, and popping 
    //    them back to a third sorted list.
    // 2) Treating the backing list as the buffer for in-place heap creation
    //    and sorting via pop & swap loop.
    //fn heap_sort() {
    //    let mut list = [1, 5, 7, 23, 45, 3, 2, 17, 56, 9, 21];
    //    let sorted = [1, 2, 3, 5, 7, 9, 17, 21, 23, 45, 56];

    //    // 1) Sorting via heap buffer (secondary storage)
    //    // Creates a heap (buffer) in O(n) time from borrowed values
    //    let mut heap = PriorityQueue::from_slice(&list);
    //    // Pops the buffered heap values to a (secondary) sorted list in O(n * log(n)) time
    //    let mut sorted_list: Vec<i32> = Vec::new();
    //    for _ in 0..list.len() {
    //        sorted_list.push(heap.pop().expect("uh oh"))
    //    }
    //    assert_eq!(sorted_list, sorted);

    //    // 2) Sorting in place
    //    PriorityQueue::heap_sort(&mut list);
    //    assert_eq!(list, sorted);

    //    let mut v = Vec::from([6, 5, 8, 3]);
    //    PriorityQueue::heap_sort(&mut v);
    //    assert_eq!(v, [3, 5, 6, 8]);

    //}

}
