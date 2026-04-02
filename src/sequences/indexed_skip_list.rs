/*! A safe, indexed skip list

# About
Skip lists are sorted, probabalistic structures made up of stacked lists of varying length to allow for truncated _O(log(n))_ navigation. Canonically linked lists are built from doubly-linked lists, but this is not a defining characteristic of the ADT. Regardless of the base list representation used, the navigational algorithm results in what is essentially a logical linked list.

Properly implemented skip lists provide _O(log(n))_ expected time complexity for search, insert, and removal operations. This provides a significant advantage over keeping sorted array- or link-based list invariants, which have _worst-case O(n)_ removal (average _O(n/2)_) temporal performance. Skip lists are also simpler than self-balancing tree structures, which are commonly used for sorted list and map structures. Skip lists also generally provide easier and finer-grained control when adapted for concurrent operations. There is a reason Java's `concurrentSkipListMap` is so popular.

# Design
This design uses `Vec`-backed storage for [SkipNode]s that contain a list (tower) of "next" values, and a single "previous" value that represent indexes within the backing vector. 

The list features a dynamic max height _h_ that is logarithmically proportional to the number of elements in the list _n_ such that _h = log(n) in the expected case_. The logarithmic growth ensures that the average search, insertion, and deletion operations remain efficient, typically with expected _O(log(n))_ time complexity.

## The Search Algorithm
The point of the search algorithm is to find the first node (or handle/position) `p` in skip list `S` that represents the largest comparable value <= to the search key `k`. This algorithm can be broken into two steps:
Step 1) loop`let candidate = p.peek()`, if `candidate >= k`, return `p`, otherwise move to `p.next()`. Repeat until `p.peek()` >= `k`.
Step 2) Drop down a level: If `S.below(p) == 0` you're at the lowest level and the search terminates.

## Visual Examples
An initial, empty skip list with one level and no data:
```text
S0: HEAD -> None
```

Inserting the first node triggers an automatic tower level, even if it ends up empty. This provides the algorithm with a starting point:
```text
S1: HEAD ----------> None
S0: HEAD -> [ 5 ] -> None
```

After inserting `['a', 'c', 'e', 'd', 'b', 'i', 'g', 'h', 'f']`, the list's `SkipNodes` might contain the following towers. Notice that its always possible to tell the last item in the list because its tower is empty. This makes sense, because inserting the last element can only point to `None`. As you can see by the index notation on the left-hand side of the table, the backing structure retains the insertion order; the backing structure remains unsorted.
```text
HEAD[0]: [1, 2, 9, 7]
a[1]: [5]
c[2]: [4, 4]
e[3]: [9]
d[4]: [3, 9]
b[5]: [2]
i[6]: []
g[7]: [8, 6, 6]
h[8]: [6]
f[9]: [7, 7, 7]
```
The towers appear to contain rather nonsensical values when printed as they exist in memory, which represents insertion order. However, if you follow the indexes from the `HEAD` node, and re-arrange the nodes into _lexicographically sorted order_, which is what the navigational algorithms in the skiplist achieve, you'd get the following towers.
```text
HEAD[0]: [1, 2, 9, 7]
a[1]: [5]
b[5]: [2]
c[2]: [4, 4]
d[4]: [3, 9]
e[3]: [9]
f[9]: [7, 7, 7]
h[8]: [6]
i[6]: []
g[7]: [8, 6, 6]
```

When you rotate the mapping 90 degrees you can start to visualize the skip list layers as logically linked lists formed by tower index placement. Notice that the towers roughly mirror the raw tower output from the previous output.
```text
L3: [ g[7] ] -> None
L2: [ f[9] ] -> [ g[7] ] -> [ i[6] ] -> None
L1: [ c[2] ] -> [ d[4] ] -> [ f[9] ] -> [ g[7] ] -> [ i[6] ] -> None
L0: [ a[1] ] -> [ b[5] ] -> [ c[2] ] -> [ d[4] ] -> [ e[3] ] -> [ f[9] ] -> [ g[7] ] -> [ h[8] ] -> [ i[6] ] -> None
```
Finally, if you extend each index reference to align with its sorted position within the list, a classical skip list diagram emerges.
```text
L3: HEAD -------------------------------------------------------------------------> [ g[7] ] -------------------------> None
L2: HEAD -------------------------------------------------------------> [ f[9] ] -> [ g[7] ] -------------> [ i[6] ] -> None
L1: HEAD -------------------------> [ c[2] ] -> [ d[4] ] -------------> [ f[9] ] -> [ g[7] ] -------------> [ i[6] ] -> None
L0: HEAD -> [ a[1] ] -> [ b[5] ] -> [ c[2] ] -> [ d[4] ] -> [ e[3] ] -> [ f[9] ] -> [ g[7] ] -> [ h[8] ] -> [ i[6] ] -> None
```

# Example code
```rust
    let mut list = SkipList::<char>::new();

    // Inserts 9 values into the skip list
    // with a consuming iterator, moving values
    // into the list
    let values = ['a', 'c', 'e', 'd', 'b', 'i', 'g', 'h', 'f'];
    for e in l1.into_iter() {
        list.insert(e)
    }

    // Illustrates that the list exists as a sorted invariant
    let sorted = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];
    for (i, e) in list.iter().enumerate() {
        assert_eq!(e, &sorted[i]);
    }

    // Illustrates the Kth function in a 0-indexed list
    assert_eq!(list.get_kth(6).unwrap(), &'g');

    // Query by range using Rust's RangeBounds semantics
    let val = ['c', 'd', 'e', 'f'];
    for (i, e) in list.range('c'..='f').enumerate() {
        assert_eq!(e, &val[i])
    }

```
*/

use rand::Rng; // For coin flips
use std::borrow::Borrow; // For passing borrowed parameters
use std::ops::{Bound, RangeBounds}; // For range iterators

const MAX_HEIGHT: usize = 32;

#[derive(Debug)]
struct SkipNode<T> {
    value: Option<T>,                  // None for sentinel
    next: [Option<usize>; MAX_HEIGHT], // forward links
    prev: Option<usize>, // back links at s0 for reverse iteration
}

pub struct SkipList<T> {
    nodes: Vec<SkipNode<T>>,
    list_height: usize,
}
impl<T: Ord> Default for SkipList<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Ord> SkipList<T> {
    pub fn new() -> Self {
        let sentinel = SkipNode {
            value: None,
            next: [None; MAX_HEIGHT],
            prev: None
        };

        Self {
            nodes: vec![sentinel],
            list_height: 1,
        }
    }

    pub fn len(&self) -> usize {
        self.nodes.len() - 1 // HEAD doesn't count!
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.len() - 1 == 0
    }

    pub fn locate<Q>(&self, key: &Q) -> Option<usize>
    where
        Q: Ord + ?Sized,
        T: Borrow<Q>,
    {
        let update = self.find_predecessors(key);
        let candidate = self.nodes[update[0]].next[0];
    
        match candidate {
            Some(idx)
                if self.nodes[idx].value.as_ref().unwrap().borrow() == key =>
            {
                Some(idx)
            }
            _ => None,
        }
    }

    pub fn contains<Q>(&self, key: &Q) -> bool 
    where 
        Q: Ord + ?Sized,
        T: Borrow<Q>
    {
        self.locate(key).is_some()
    }

    pub fn insert(&mut self, value: T)
    where
        T: Ord,
    {
        let height = self.random_height();
        let update = self.find_predecessors(&value);
    
        // ... height adjustment logic ...
    
        let new_index = self.nodes.len();
        
        // 1. SET NEW NODE'S PREV
        // update[0] is the index of the node immediately before the new one at Level 0.
        let predecessor_idx = update[0];
        
        // 2. GET NEW NODE'S NEXT (at Level 0)
        // This is the node that will now need to point back to us.
        let successor_idx = self.nodes[predecessor_idx].next[0];
    
        self.nodes.push(SkipNode {
            value: Some(value),
            next: [None; MAX_HEIGHT],
            prev: Some(predecessor_idx), // Points back to A
        });
    
        for (level, _) in update.iter().enumerate().take(height) {
        //for level in 0..height {
            let prev_idx = update[level];
            self.nodes[new_index].next[level] = self.nodes[prev_idx].next[level];
            self.nodes[prev_idx].next[level] = Some(new_index);
        }
    
        // 3. UPDATE SUCCESSOR'S PREV
        // If there is a node after us, it must now point back to the new node.
        if let Some(next_idx) = successor_idx {
            self.nodes[next_idx].prev = Some(new_index);
        }
    }

    /// Removes and returns the value for a given key, if it exists in the list.
    pub fn remove<Q>(&mut self, key: &Q) -> Option<T>
    where
        Q: Ord + ?Sized,
        T: Borrow<Q>,
    {
        let mut update_target = self.find_predecessors(key);
    
        // 1. Identify the target index
        let target = match self.nodes[update_target[0]].next[0] {
            //Some(idx) if self.nodes[idx].value.as_ref().map_or(false, |v| v.borrow() == key) => idx,
            Some(idx) if self.nodes[idx].value.as_ref().is_some_and(|v| v.borrow() == key) => idx,
            _ => return None,
        };
    
        // 2. PRE-FETCH predecessors for the node that WILL move
        // We do this BEFORE swap_remove so the indices in the skip list are still valid
        let last_idx = self.nodes.len() - 1;
        let mut update_moved = [0usize; MAX_HEIGHT];
        if target != last_idx {
            let moved_val = self.nodes[last_idx].value.as_ref().unwrap().borrow();
            update_moved = self.find_predecessors(moved_val);
        }
    
        // 3. Logical Rewiring (Remove target from the chain)
        if let Some(next_idx) = self.nodes[target].next[0] {
            self.nodes[next_idx].prev = self.nodes[target].prev;
        }
        for (level, val) in update_target.iter_mut().enumerate().take(self.list_height) {
        //for level in 0..self.list_height {
            if self.nodes[*val].next[level] == Some(target) {
                self.nodes[*val].next[level] = self.nodes[target].next[level];
            }
        }
    
        // 4. Physical Swap-Remove
        let removed_node = self.nodes.swap_remove(target);
    
        // 5. Fix references for the node that just moved into 'target'
        if target < self.nodes.len() {
            // Any node in 'update_moved' that pointed to 'last_idx' must now point to 'target'
            for level in 0..self.list_height {
                if update_moved[level] == last_idx {
                    // Rare case: the moved node's predecessor was itself the target
                    // We handle this by using the updated pointers from the target's removal
                    update_moved[level] = update_target[level];
                }
                
                if self.nodes[update_moved[level]].next[level] == Some(last_idx) {
                    self.nodes[update_moved[level]].next[level] = Some(target);
                }
            }
            // Update the successor of the moved node to point back to the new index
            if let Some(next_idx) = self.nodes[target].next[0] {
                self.nodes[next_idx].prev = Some(target);
            }
        }
    
        // ... clean up height ...
        removed_node.value
    }

    /// Returns the Kth value in the list, if it exists.
    pub fn get_kth(&self, k: usize) -> Option<&T> {
        let mut idx = self.nodes[0].next[0];
        let mut i = 0;
    
        while let Some(current) = idx {
            if i == k {
                return self.nodes[current].value.as_ref();
            }
    
            idx = self.nodes[current].next[0];
            i += 1;
        }
    
        None
    }

    /// Returns an inclusive iterator over a range of values 
    /// in the list from `start` to `end`.
    pub fn range<Q, R>(&self, range: R) -> RangeIter<'_, T, Q, R>
    where
        Q: Ord + ?Sized,
        T: Borrow<Q>,
        R: RangeBounds<Q>,
    {
        // --- FIND FRONT ---
        let front = match range.start_bound() {
            Bound::Included(start) => self.nodes[self.find_predecessors(start)[0]].next[0],
            Bound::Excluded(start) => {
                let idx = self.nodes[self.find_predecessors(start)[0]].next[0];
                if let Some(i) = idx {
                    if self.nodes[i].value.as_ref().unwrap().borrow() == start {
                        self.nodes[i].next[0]
                    } else { Some(i) }
                } else { None }
            }
            Bound::Unbounded => self.nodes[0].next[0],
        };
    
        // --- FIND BACK ---
        let back = match range.end_bound() {
            Bound::Included(end) => {
                // Find predecessors of 'end'. 
                // If the element at the end of the search IS 'end', that's our back.
                // If not, the predecessor itself is our back.
                let update = self.find_predecessors(end);
                let candidate = self.nodes[update[0]].next[0];
                if let Some(idx) = candidate {
                    if self.nodes[idx].value.as_ref().unwrap().borrow() == end {
                        Some(idx)
                    } else {
                        // Predicate check: Ensure we aren't returning the sentinel (idx 0)
                        if update[0] == 0 { None } else { Some(update[0]) }
                    }
                } else if update[0] == 0 { None } else { Some(update[0]) }
            }
            Bound::Excluded(end) => {
                let update = self.find_predecessors(end);
                if update[0] == 0 { None } else { Some(update[0]) }
            }
            Bound::Unbounded => {
                // To find the absolute end, we find predecessors for a "theoretically infinite" value
                // or simply walk the tallest tower to the end.
                let mut curr = 0;
                for level in (0..self.list_height).rev() {
                    while let Some(next_idx) = self.nodes[curr].next[level] {
                        curr = next_idx;
                    }
                }
                if curr == 0 { None } else { Some(curr) }
            }
        };
    
        RangeIter {
            list: self,
            front,
            back,
            range,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        // Walk the express lanes to find the very last node in O(log n)
        let mut tail = 0;
        for level in (0..self.list_height).rev() {
            while let Some(next_idx) = self.nodes[tail].next[level] {
                tail = next_idx;
            }
        }
    
        Iter {
            list: self,
            next: self.nodes[0].next[0], // First node after sentinel
            prev: if tail == 0 { None } else { Some(tail) },
        }
    }

    // Utility functions
    ////////////////////

    fn random_height(&self) -> usize {
        let mut level = 1;
        let mut rng = rand::rng();
        while level < MAX_HEIGHT && rng.random::<bool>() {
            level += 1;
        }
        level
    }

    fn find_predecessors<Q>(&self, key: &Q) -> [usize; MAX_HEIGHT]
    where
        Q: Ord + ?Sized,
        T: Borrow<Q>,
    {
        let mut update = [0usize; MAX_HEIGHT];
        let mut idx = 0;
    
        for level in (0..self.list_height).rev() {
            loop {
                match self.nodes[idx].next[level] {
                    None => break,
                    Some(next_idx) => {
                        let val = self.nodes[next_idx].value.as_ref().unwrap();
                        if val.borrow() >= key {
                            break;
                        }
                        idx = next_idx;
                    }
                }
            }
            update[level] = idx;
        }
    
        update
    }

    //fn fix_moved_node_references(&mut self, new_idx: usize, old_idx: usize) {
    //    // We need to find the neighbors of the node that is now at new_idx
    //    // so we can tell them "I moved!"
    //    let val = self.nodes[new_idx].value.as_ref().unwrap();
    //    let update = self.find_predecessors(val);
    //
    //    // Update predecessors: they currently point to old_idx, they should point to new_idx
    //    for level in 0..MAX_HEIGHT {
    //        if self.nodes[update[level]].next[level] == Some(old_idx) {
    //            self.nodes[update[level]].next[level] = Some(new_idx);
    //        } else {
    //            // Since towers are vertical, if we don't find a match at this level, 
    //            // we might not find them higher up, but check all to be safe.
    //        }
    //    }
    //
    //    // Update successor: the node after the moved node needs to update its 'prev'
    //    if let Some(next_idx) = self.nodes[new_idx].next[0] {
    //        self.nodes[next_idx].prev = Some(new_idx);
    //    }
    //}
}

pub struct Iter<'a, T> {
    list: &'a SkipList<T>,
    next: Option<usize>,
    prev: Option<usize>
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.next?;
        let value = self.list.nodes[idx].value.as_ref()?;

        if self.next == self.prev {
            self.next = None;
            self.prev = None;
        } else {
            self.next = self.list.nodes[idx].next[0];
        }
        Some(value)
    }
}
impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let idx = self.prev?;
        let value = self.list.nodes[idx].value.as_ref()?;

        if self.prev == self.next {
            self.next = None;
            self.prev = None;
        } else {
            let prev = self.list.nodes[idx].prev;
            // Sentinel check: don't yield index 0
            self.prev = if prev == Some(0) { None } else { prev };
        }
        Some(value)
    }
}

pub struct RangeIter<'a, T, Q, R> 
where 
    Q: ?Sized,
    R: RangeBounds<Q>,
{
    list: &'a SkipList<T>,
    front: Option<usize>, // Moves forward
    back: Option<usize>,  // Moves backward
    range: R,
    _marker: std::marker::PhantomData<Q>,
}
impl<'a, T, Q, R> Iterator for RangeIter<'a, T, Q, R>
where
    Q: Ord + ?Sized,
    T: Borrow<Q>,
    R: RangeBounds<Q>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.front?;
        
        // Boundary Check
        let value = self.list.nodes[idx].value.as_ref().unwrap();
        if !self.range.contains(value.borrow()) {
            self.front = None;
            return None;
        }

        // Meet/Cross Check: If front matches back, this is the last element
        if self.front == self.back {
            self.front = None;
            self.back = None;
        } else {
            self.front = self.list.nodes[idx].next[0];
        }

        Some(value)
    }
}
impl<'a, T, Q, R> DoubleEndedIterator for RangeIter<'a, T, Q, R>
where
    Q: Ord + ?Sized,
    T: Borrow<Q>,
    R: RangeBounds<Q>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let idx = self.back?;

        // Boundary Check
        let value = self.list.nodes[idx].value.as_ref().unwrap();
        if !self.range.contains(value.borrow()) {
            self.back = None;
            return None;
        }

        // Meet/Cross Check
        if self.back == self.front {
            self.back = None;
            self.front = None;
        } else {
            // Move back, but ensure we don't land on the sentinel (idx 0)
            let prev = self.list.nodes[idx].prev;
            self.back = if prev == Some(0) { None } else { prev };
        }

        Some(value)
    }
}

impl<'a, T: Ord> IntoIterator for &'a SkipList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[test]
fn one() {
    let mut list = SkipList::<char>::new();

    // Tests basic housekeeping on empty list
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
    assert!(!list.contains(&'z'));

    // Inserts 9 values into the skip list
    // with a consuming iterator, moving values
    // into the list
    let values = ['a', 'c', 'e', 'd', 'b', 'i', 'g', 'h', 'f'];
    for e in values.into_iter() {
        list.insert(e)
    }

    // Tests that len gets updated properly
    assert_eq!(list.len(), 9);
    assert!(list.contains(&'g'));

    // Tests basic ordering and iteration
    // Basic iteration with iter()
    // Clippy wants enumerate instead of external loop counter
    let sorted = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];
    for (i, e) in list.iter().enumerate() {
        assert_eq!(e, &sorted[i]);
    }
    // Double-ended iteration with rev()
    // Clippy wants saturating_sub instead of loop counter
    let mut i = 8;
    for e in list.iter().rev() {
        assert_eq!(e, &sorted[i]);
        if i > 0 {
            i = i.saturating_sub(1)
        };
    }
    // Or if you wanna be fancy about it
    // zip() stops as soon as one iterator ends, 
    // eliminating the need for an overflow check
    for (e, i) in list.iter().rev().zip((0..=8).rev()) {
        assert_eq!(e, &sorted[i]);
    }
    // Iterator inferance using the IntoIter impl
    let mut i = 0;
    #[allow(clippy::explicit_counter_loop)]
    for e in &list {
        assert_eq!(e, &sorted[i]);
        i += 1;
    }

    // Tests the Kth function in a 0-indexed list
    assert_eq!(list.get_kth(6).unwrap(), &'g');

    // Tests the range function
    // NOTE: char is Copy so you dont strictly need to borrow 
    // when setting range bounds, these tests illustrate both
    // borrowing and not; Note that each bounds must match
    // so no (&'a'..'f'), only ('a'..'f') or (&'a'..&'f')
    // Midlist (exclusive)
    let val = ['c', 'd', 'e'];
    //for (i, e) in list.range(&'c', &'f').enumerate() {
    for (i, e) in list.range('c'..'f').enumerate() {
        assert_eq!(e, &val[i])
    }
    // Midlist (inclusive)
    let val = ['c', 'd', 'e', 'f'];
    //for (i, e) in list.range(&'c', &'f').enumerate() {
    for (i, e) in list.range('c'..='f').enumerate() {
        assert_eq!(e, &val[i])
    }
    // Start of list
    let val = ['a', 'b', 'c', 'd', 'e', 'f'];
    //for (i, e) in list.range(&'a', &'f').enumerate() {
    for (i, e) in list.range(..&'f').enumerate() {
        assert_eq!(e, &val[i])
    }
    // End of list
    let val = ['e', 'f', 'g', 'h', 'i'];
    for (i, e) in list.range(&'e'..).enumerate() {
        assert_eq!(e, &val[i])
    }

    // Tests removal
    list.remove(&'e');
    list.remove(&'a');
    assert!(!list.contains(&'e'));
    assert!(!list.contains(&'a'));
    let l2 = ['b', 'c', 'd', 'f', 'g', 'h', 'i'];
    for (val, i) in list.iter().zip(0..=6) {
        assert_eq!(val, &l2[i]);
    }

    // Cant remove what isn't there!
    assert!(list.remove(&'z').is_none());

    // Everything below here is for display/debug purposes
    //////////////////////////////////////////////////////

    // Debug print
    // Prints the tower contents
    println!("Tower contents by insertion order, NOT sorted order:");
    for (i, e) in list.nodes.iter().enumerate() {
        // Collect only the Some values into a new Vec
        let values: Vec<_> = e.next.iter().filter_map(|&x| x).collect();
        if let Some(val) = e.value {
            let v = &val.to_string();
            println!("{v}[{i}]: {values:?}");
        } else {
            println!("HEAD[0]: {values:?}");
        }
    }
    println!();

    // Debug print
    // Prints the sorted values in the list
    print!("List values:\n   ");
    for e in list.iter() {
        print!("{e:#?} ")
    }
    println!();

    //panic!();

}
