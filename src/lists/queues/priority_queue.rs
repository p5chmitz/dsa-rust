/////////////////////////////////////////
/** A sorted, Vec-based priority queue */
/////////////////////////////////////////

// A sorted, array-based priority queue
mod sorted_list {
pub use crate::lists::queues::traits::PriorityQueue; // Re-exports the trait

pub struct Entry<K, V> {
    key: K,
    value: V,
}
impl<K, V> Entry<K, V> {
    // Creates a new Entry from a key/value pair
    fn new(key: K, value: V) -> Entry<K, V> {
        Entry {
            key,
            value,
        } 
    }
}
/** Defines a MIN queue where the key's value is inversely proportional to its 
priority; Based on a sorted Vec of Entry<K, V> where K is the key, 
and V is the value ("payload") */
pub struct SortedListQueue<K, V> {
    pub data: Vec<Entry<K, V>>
}
impl<K, V> SortedListQueue<K, V> {
    // Creates a new, empty priority queue structure
    pub fn new() -> SortedListQueue<K, V> {
        SortedListQueue {
            data: Vec::with_capacity(0)
        }
    }
}
impl<K, V> PriorityQueue<K, V> for SortedListQueue<K, V>
where K: Ord {

    type Entry = Entry<K, V>;

    //NOTE: Provides a wrapper for Vec::insert() which runs in O(n) time
    fn insert(&mut self, key: K, value: V) -> Result<(), Box<dyn std::error::Error>> {
        if Self::check_key(&key) {
            let mut insertion_index = self.data.len(); 
            // Finds the correct insertion index
            for (i, e) in self.data.iter().enumerate() {
                if key >= e.key {
                    insertion_index = i; 
                    break;
                }
            }
            let entry = Entry::new(key, value);
            self.data.insert(insertion_index, entry); // Actually calls Vec::insert()
            Ok(())
        } else {
            Err("Invalid key".into())
        }
    }

    //NOTE: Provides a wrapper for core::slice::last() which I THINK runs in O(1)...
    fn peek(&self) -> Option<&V> {
        if let Some(v) = self.data.last() {
            Some(&v.value)
        } else { None }
    }

    //NOTE: Provides a wrapper for Vec::pop() which runs in O(1) time
    fn dequeue(&mut self) -> Option<V> {
        if let Some(v) = self.data.pop() {
            return Some(v.value)
        } else { None }
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn compare(one: Self::Entry, two: Self::Entry) -> isize {
        if one.key < two.key { -1 }
        else if one.key == two.key { 0 }
        else { 1 }
    }

    fn check_key(key: &K) -> bool {
        key == key 
    }

}

}

#[test]
pub fn example() {
    use crate::lists::queues::priority_queue::sorted_list::{
        PriorityQueue, 
        SortedListQueue
    };
    
    // Instantiates new list, declares the K and V types
    let mut list: SortedListQueue<usize, &str> = SortedListQueue::new();

    // Pushes a bunch of values with an associated key priority to the list
    list.insert(3, "Peter").ok();
    list.insert(5, "Bobson").ok();
    list.insert(2, "Brain").ok();
    list.insert(4, "Dingus").ok();
    list.insert(6, "Dorkus").ok();

    // Checks that the list is taking entries properly,
    // and that the peek() operation matches expectations
    assert_eq!(list.size(), 5);
    assert_eq!(list.peek(), Some("Brain").as_ref());

    // Creates a "sorted" list of dequeued items
    let mut queue: Vec<&str> = Vec::new();
    // Using the while loop avoids having to deal with partial moves in a for loop
    while !list.is_empty() {
        if let Some(v) = list.dequeue() {
            queue.push(v)
        }
    }

    // Checks that the final result of the queue logic is correct
    assert_eq!(queue, vec!["Brain", "Peter", "Dingus", "Bobson", "Dorkus"])

}
