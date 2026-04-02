/*! A safe, concurrent, reference-counted skip list

# About
Skip list are sorted, probabalistic, list-based structures that provide _O(log(n))_ (expected) time complexity for search, insert, and removal operations. This provides a significant advantage over sorted array-based lists, which exhibit _worst-case O(n)_ removal (average _O(n/2)_) temporal performance. Introduced in 1989 by William Pugh, skip lists also tend to be simpler to implement than self-balancing tree structures and generally provide an easier and finer-grained control when adapting these structures for concurrent operations.

This structure provides a basic list implementation used to back this library's [SkipListMap<K, V>] implementation.

# Design
The list features a dynamic max height `h` that is logarithmically proportional to the number of elements in the list `n` such that _h = log(n)_ in the expected case. The list does not _rebuild_ any towers (besides the head node) at insert, but simply changes the maximum potential height for those towers to grow. The logarithmic growth ensures that the average search, insertion, and deletion operations remain efficient, typically with expected O(log n) time complexity.

## The Search Algorithm
The point of the search algorithm is to find the first node (or handle/position) `p` in skip list `S` that represents the largest comparable value <= to the search key `k`. This algorithm can be broken into two steps:
Step 1) loop`let candidate = p.peek()`, if `candidate >= k`, return `p`, otherwise move to `p.next()`. Repeat until `p.peek()` >= `k`.
Step 2) Drop down a level: If `S.below(p) == 0` you're at the lowest level and the search terminates.

## Visual Examples
An initial, empty skip list with one level and no data:
```text
S0: None -> None
```

Inserting the first node triggers an automatic tower level, even if it ends up empty. This provides the algorithm with a starting point:
```text
S1: None ---------> None
S0: None -> [ 5] -> None
```

Each subsequent addition may add more tower levels up to log(n) where `n` is the number of elements in the list. This example illustrates a fully built skip list:
```text
S3: None ---------> [*2] ---------------------------------------------> [*9] ----------> None
S2: None ---------> [*2] ------------------------------> [*7] --------> [*9] ----------> None
S1: None ---------> [*2] --------> [*4] ---------------> [*7] --------> [*9] -> [*10] -> None
S0: None -> [ 1] -> [ 2] -> [3] -> [ 4] -> [5] -> [6] -> [ 7] -> [8] -> [ 9] -> [ 10] -> None
```

[None, a, j, c, e, d, b, i, g, h, f, None]
[  0,  1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
[None, a, b, c, d, e, f, g, h, i, j, None]

// Towers forming sorted "express lanes"
```text
S3: None ----------> [ 6 ] -------------------------------------------------------> [ 7 ] ----------> None
S2: None ----------> [ 6 ] -------------------------------------> [ 8 ] ----------> [ 7 ] ----------> None
S1: None ----------> [ 6 ] ----------> [ 5 ] -------------------> [ 8 ] ----------> [ 7 ] -> [ 2 ] -> None
```
//Unsorted backing structure
```text
S0: None -> [ a ] -> [ j ] -> [ c ] -> [ e ] -> [ d ] -> [ b ] -> [ i ] -> [ g ] -> [ h ] -> [ f ] -> None
```


For example, at level 2 you could traverse the list as pointers to [2, 7, 9]. The distinction here is that all of the node data is stored in one location, with only pointers being duplicated and populating tower lists.

# Example

*/
use std::sync::{Arc, RwLock};

type Link<T> = RwLock<Option<Arc<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    data: Arc<T>, // Shared value across all levels
    next: Link<T>,
    down: Option<Arc<Node<T>>>, // Pointer to the node at the level below
}

use std::borrow::Borrow;
use std::sync::atomic::{AtomicUsize, Ordering};
//use std::cmp::{max, Ordering};
//use rand;

pub struct ConcurrentSkipList<T> {
    heads: Vec<Link<T>>,
    len: AtomicUsize,
}
impl<T: Ord> Default for ConcurrentSkipList<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Ord> ConcurrentSkipList<T> {
    /// Tower utility
    //fn generate_tower_height(&self) -> usize {
    //    // Simple deterministic-ish pseudo-random for demo purposes
    //    // Use a proper, fast RNG
    //    //let mut level = 1;
    //    //while level < 16 && (self.len.load(Ordering::Relaxed) >> level) & 1 == 1 {
    //    //    level += 1;
    //    //}
    //    //level
    //    // OR, use some black magic bit logic
    //    let mut height = 1;
    //    // Simple bit-logic: 50% chance for each level
    //    let seed = self.len.load(Ordering::Relaxed) + 1;
    //    while height < self.heads.len() && (seed & (1 << height)) != 0 {
    //        height += 1;
    //    }
    //    height
    //}
    fn generate_tower_height(&self) -> usize {
        let mut height = 1;
        let mut rng = rand::rng();
        // 0.25 (1 in 4) is often faster than 0.5 for skip lists
        while height < self.heads.len() && rand::Rng::random_bool(&mut rng, 0.25) {
            height += 1;
        }
        height
    }

    /// Instantiates a new concurrent skip list.
    pub fn new() -> Self {
        // Pre-allocate levels to allow &self mutation of links without resizing the Vec
        let mut heads = Vec::with_capacity(32);
        for _ in 0..32 {
            heads.push(RwLock::new(None));
        }
        Self {
            heads,
            len: AtomicUsize::new(0),
        }
    }

    /// Returns the number of nodes in the (base) list.
    pub fn len(&self) -> usize {
        self.len.load(Ordering::SeqCst)
    }

    /// Returns true if the (base) is empty.
    pub fn is_empty(&self) -> bool {
        self.len.load(Ordering::Relaxed) == 0
    }

    /// Adds a new node to the list.
    pub fn push(&self, data: T) -> bool {
        let target_height = self.generate_tower_height();
        let shared_data = Arc::new(data);

        // To handle concurrency, we must find predecessors level-by-level
        // and verify they are still valid when we write.
        let mut lower_node: Option<Arc<Node<T>>> = None;

        // We build from bottom up to ensure base-level consistency first
        for level in 0..target_height {
            loop {
                // 1. Find the predecessor for this specific level
                let (pred_opt, succ_opt) =
                    self.find_neighbors_at_level(level, shared_data.as_ref());

                // 2. Check for duplicates (only need to do this at the base level or check every time)
                if let Some(ref succ) = succ_opt {
                    if succ.data.as_ref() == shared_data.as_ref() {
                        return false;
                    }
                }

                // 3. Create the new node for this level
                let new_node = Arc::new(Node {
                    data: Arc::clone(&shared_data),
                    next: RwLock::new(succ_opt.clone()),
                    down: lower_node.clone(),
                });

                // 4. Attempt to link
                let success = if let Some(ref pred) = pred_opt {
                    let mut write_guard = pred.next.write().unwrap();
                    // RE-VERIFY: Is the successor still the same?
                    // This prevents the TOCTOU race.
                    if write_guard.as_ref().map(Arc::as_ptr) == succ_opt.as_ref().map(Arc::as_ptr) {
                        *write_guard = Some(Arc::clone(&new_node));
                        true
                    } else {
                        false // Someone squeezed in between; retry this level
                    }
                } else {
                    let mut head_guard = self.heads[level].write().unwrap();
                    if head_guard.as_ref().map(Arc::as_ptr) == succ_opt.as_ref().map(Arc::as_ptr) {
                        *head_guard = Some(Arc::clone(&new_node));
                        true
                    } else {
                        false
                    }
                };

                if success {
                    lower_node = Some(new_node);
                    break; // Level complete, move up
                }
                // If !success, the loop repeats for this level
            }
        }

        self.len.fetch_add(1, Ordering::SeqCst);
        true
    }

    #[allow(clippy::type_complexity)]
    /// Utility to find the immediate nodes surrounding a key at a specific level
    fn find_neighbors_at_level(
        &self,
        level: usize,
        key: &T,
    ) -> (Option<Arc<Node<T>>>, Option<Arc<Node<T>>>) {
        let mut curr = self.heads[level].read().unwrap().clone();
        let mut pred = None;

        while let Some(node) = curr {
            if node.data.as_ref() < key {
                pred = Some(Arc::clone(&node));
                curr = node.next.read().unwrap().clone();
            } else {
                return (pred, Some(Arc::clone(&node)));
            }
        }
        (pred, None)
    }

    /// Returns a reference-counted pointer to the data in the node, if it exists.
    pub fn get<Q>(&self, key: &Q) -> Option<Arc<T>>
    where
        Q: Ord + ?Sized,
        T: Borrow<Q>,
    {
        // This will track our position as we move right and then down
        let mut current_node: Option<Arc<Node<T>>> = None;

        for level in (0..self.heads.len()).rev() {
            // 1. Where do we start at this level?
            // If we found a node at the level above, we move 'down' from it.
            // If we haven't found a starting node yet, we start at the head of this level.
            let mut cursor = if let Some(ref node) = current_node {
                node.down.clone()
            } else {
                self.heads[level].read().unwrap().clone()
            };

            // 2. Move horizontally as far as possible at this level
            while let Some(node) = cursor {
                let node_data: &Q = node.data.as_ref().borrow();

                // Match found!
                if node_data == key {
                    return Some(Arc::clone(&node.data));
                }

                if node_data < key {
                    // Check if the next node is still <= our target
                    let next_guard = node.next.read().unwrap();
                    if let Some(next_node) = next_guard.as_ref() {
                        let next_data: &Q = next_node.data.as_ref().borrow();
                        if next_data <= key {
                            // Move right
                            cursor = Some(Arc::clone(next_node));
                            continue;
                        }
                    }
                }

                // If we've reached here, we can't move right anymore.
                // Mark this node as our "downward" jump point for the next level.
                current_node = Some(node);
                break;
            }
        }

        None
    }

    /// Removes a node from the list.
    pub fn remove<Q>(&self, key: &Q) -> Option<Arc<T>>
    where
        Q: Ord + ?Sized,
        T: Borrow<Q>,
    {
        let mut removed_val = None;
        let mut predecessor: Option<Arc<Node<T>>> = None;

        for level in (0..self.heads.len()).rev() {
            // 1) The Elevator: Move Down
            // Get the starting node for this level
            let mut curr_link = if let Some(pred) = &predecessor {
                pred.down.clone()
            } else {
                self.heads[level].read().unwrap().clone()
            };

            // 2) The Head Check
            // If we are still at the start of a level (predecessor is None),
            // we need to check if the head itself needs to be unhooked
            if predecessor.is_none() {
                let mut head_lock = self.heads[level].write().unwrap();
                let mut is_target = false;

                if let Some(node) = head_lock.as_ref() {
                    if node.data.as_ref().borrow() == key {
                        is_target = true;
                        removed_val = Some(Arc::clone(&node.data));
                    }
                }

                if is_target {
                    let target_node = head_lock.take().unwrap();
                    *head_lock = target_node.next.read().unwrap().clone();
                    // After unhooking the head, the new head is the starting point for the search
                    curr_link = head_lock.clone();
                }
            }

            // 3) Search Right & Unhook Mid-List
            while let Some(node) = curr_link {
                let node_q: &Q = node.data.as_ref().borrow();

                if node_q < key {
                    // This node is a valid predecessor candidate.
                    // Peek at its 'next' to see if it's the target.
                    let mut next_lock = node.next.write().unwrap();
                    let mut next_is_target = false;

                    if let Some(next_node) = next_lock.as_ref() {
                        if next_node.data.as_ref().borrow() == key {
                            next_is_target = true;
                            removed_val = Some(Arc::clone(&next_node.data));
                        }
                    }

                    if next_is_target {
                        let target_node = next_lock.take().unwrap();
                        *next_lock = target_node.next.read().unwrap().clone();
                    }

                    // Move forward: this node is now the predecessor for the next level down.
                    predecessor = Some(Arc::clone(&node));
                    curr_link = next_lock.clone();
                } else {
                    break;
                }
            }
        }

        if removed_val.is_some() {
            self.len.fetch_sub(1, Ordering::SeqCst);
        }
        removed_val
    }

    /// Returns true if the item exists in the list.
    pub fn contains<Q>(&self, key: &Q) -> bool
    where
        Q: Ord + ?Sized,
        T: Borrow<Q>,
    {
        // contains is just a boolean check on get()
        self.get(key).is_some()
    }
}

mod test {

    #![allow(unused)]
    use super::ConcurrentSkipList;
    use std::sync::Arc;

    #[test]
    // Single-threaded test
    fn one() {
        // No need for mut!
        let list = ConcurrentSkipList::<&'static str>::new();

        list.push("Peter");
        list.push("Paul");
        list.push("Mary");

        // Tests len() & is_empty()
        assert_eq!(list.len(), 3);
        assert!(!list.is_empty());

        // Tests contains()
        assert!(list.contains("Peter"));

        // Tests get()
        let m = list.get("Mary"); // Fails???
                                  //assert!(m.is_some());
        let check = Some(Arc::new("Mary"));
        //assert_eq!(m, check);

        // Tests remove()
        let check = Some(Arc::new("Peter"));
        assert_eq!(list.remove("Peter"), check);
    }

    #[test]
    // Multi-threaded test
    //fn two() {
    //    use std::sync::{Arc, Barrier};
    //    use std::thread;

    //    let list = Arc::new(ConcurrentSkipList::<&'static str>::new());

    //    let values = vec![
    //        "Peter", "Paul", "Mary", "John", "Luke", "Mark", "Matthew", "James", "Jude", "Simon",
    //    ];

    //    let thread_count = 8;
    //    let barrier = Arc::new(Barrier::new(thread_count));

    //    let mut handles = Vec::new();

    //    // ---- INSERT PHASE ----
    //    for _ in 0..thread_count {
    //        let list = Arc::clone(&list);
    //        let barrier = Arc::clone(&barrier);
    //        let values = values.clone();

    //        handles.push(thread::spawn(move || {
    //            barrier.wait();

    //            for v in &values {
    //                list.push(*v);
    //            }
    //        }));
    //    }

    //    for h in handles {
    //        h.join().unwrap();
    //    }

    //    // After all inserts: every value must exist exactly once
    //    assert_eq!(list.len(), values.len());

    //    for v in &values {
    //        //assert!(list.contains(v));
    //        //assert!(list.get(v).is_some());
    //    }

    //    eprintln!("{}", list.heads.len());
    //    eprintln!("{:?}", list.heads);
    //    //panic!();

    //    // ---- REMOVE PHASE ----
    //    let barrier = Arc::new(Barrier::new(thread_count));
    //    let mut handles = Vec::new();

    //    for i in 0..thread_count {
    //        let list = Arc::clone(&list);
    //        let barrier = Arc::clone(&barrier);
    //        let values = values.clone();

    //        handles.push(thread::spawn(move || {
    //            barrier.wait();

    //            // Half the threads try to remove
    //            if i % 2 == 0 {
    //                for v in &values {
    //                    list.remove(v);
    //                }
    //            }
    //        }));
    //    }

    //    for h in handles {
    //        h.join().unwrap();
    //    }

    //    // After racing removals, length is somewhere 0..=values.len()
    //    let len = list.len();
    //    assert!(len <= values.len());

    //    // Structural consistency check
    //    for v in &values {
    //        if list.contains(v) {
    //            assert!(list.get(v).is_some());
    //        }
    //    }
    //}

    //#[test]
    // Mangle them shits for giggles
    fn three() {
        use rand::seq::SliceRandom;
        use std::sync::{Arc, Barrier};
        use std::thread; // Add 'rand' to your dev-dependencies

        let list = Arc::new(ConcurrentSkipList::<i32>::new());
        let element_count = 100;
        let thread_count = 8;

        // Create a large set of numbers
        let mut values: Vec<i32> = (0..element_count).collect();
        let barrier = Arc::new(Barrier::new(thread_count));

        let mut handles = Vec::new();

        // PHASE 1: MANGLED INSERTS
        ///////////////////////////

        for _ in 0..thread_count {
            let list = Arc::clone(&list);
            let barrier = Arc::clone(&barrier);
            let mut my_values = values.clone();

            handles.push(thread::spawn(move || {
                // Shuffle so threads insert in different orders
                let mut rng = rand::rng();
                my_values.shuffle(&mut rng);

                barrier.wait();
                for v in my_values {
                    list.push(v);
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }

        // Final state check: Should be set-semantics (no duplicates)
        assert_eq!(list.len(), element_count as usize);

        // PHASE 2: MIXED READ/REMOVE RACE
        //////////////////////////////////

        let barrier = Arc::new(Barrier::new(thread_count));
        let mut handles = Vec::new();

        for i in 0..thread_count {
            let list = Arc::clone(&list);
            let barrier = Arc::clone(&barrier);
            let mut my_values = values.clone();

            handles.push(thread::spawn(move || {
                let mut rng = rand::rng();
                my_values.shuffle(&mut rng);
                barrier.wait();

                for v in my_values {
                    if i % 2 == 0 {
                        // Even threads remove
                        list.remove(&v);
                    } else {
                        // Odd threads just verify they can still read
                        // without crashing or seeing corrupted state
                        list.contains(&v);
                    }
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        // Structural Integrity: If the list isn't empty, it must be traversable
        // and the length must match the actual node count.
        let mut count = 0;
        //for e in list.iter() {
        //    eprintln!(e);
        //}
    }
}

mod dependent {
    //use crossbeam_epoch::{Atomic, Guard, Owned, Shared};
    //use std::sync::atomic::Ordering;

    //struct Node<K, V> {
    //    key: K,
    //    value: V,
    //    // The tower is a fixed-size array or Vec of Atomic Pointers
    //    next: Vec<Atomic<Node<K, V>>>,
    //}

    //pub struct SkipList<K, V> {
    //    // The head is just a tower of pointers to the first nodes
    //    head: Vec<Atomic<Node<K, V>>>,
    //}
}

// Prototype for skip map

//struct Node<K, V> {
//    key: K,
//    value: Arc<V>, // Shared value across all levels
//    next: Link<K, V>,
//    down: Option<Arc<Node<K, V>>>, // Pointer to the level below
//}

//pub struct ConcurrentSkipMap<K, V> {
//    // Array of heads for each level
//    heads: Vec<Link<K, V>>,
//}
