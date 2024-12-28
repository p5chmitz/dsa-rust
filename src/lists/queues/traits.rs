/** Defines a Priority Queue structure */
pub trait PriorityQueue<K, V>
where
    // Priority queue keys must retain a total order scheme
    K: Ord,
{
    type Entry;

    /** Takes a key/value pair, checks for key validity, 
    creates an Entry, and returns a Result */
    fn insert(&mut self, key: K, value: V) -> Result<(), Box<dyn std::error::Error>>;

    // Returns an immutable reference to the min/max Entry
    fn peek(&self) -> Option<&V>;

    // Returns and removes the min/max Entry
    fn dequeue(&mut self) -> Option<V>;

    // Returns the size of the priority queue
    fn size(&self) -> usize;
    
    // Checks if the list is empty
    fn is_empty(&self) -> bool;

    // Takes two entries, returns -1 if one < two, 0 if one == two, and 1 if one > two
    fn compare(one: Self::Entry, two: Self::Entry) -> isize;

    //NOTE: Is this needed?
    // Checks if a provided key is valid by testing if it can be compared with itself
    fn check_key(key: &K) -> bool;

}

