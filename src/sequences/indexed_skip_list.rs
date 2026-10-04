/*! A safe, indexed skip list

# About
Skip lists are sorted, probabalistic structures made up of logically stacked lists of varying length to allow for truncated _O(log(n))_ navigation. Canonically linked lists are built from doubly-linked lists, but this is not a defining characteristic of the ADT. Regardless of the base list representation used, the navigational algorithm results in what is essentially a logical linked list.

Properly implemented skip lists provide _O(log(n))_ expected time complexity for search, insert, and removal operations. This provides a significant advantage over keeping sorted array- or link-based list invariants, which have _worst-case O(n)_ removal (average _O(n/2)_) temporal performance. Skip lists are also simpler than self-balancing tree structures, which are commonly used for sorted list and map structures. Skip lists also generally provide easier and finer-grained control when adapted for concurrent operations. There is a reason Java's `concurrentSkipListMap` is so popular.

# Design
This design uses `Vec`-backed storage for [SkipNode]s that contain a list (tower) of "next" values, and a single "previous" value that represent indexes within the backing vector.

The list features a dynamic max height _h_ that is logarithmically proportional to the number of elements in the list _n_ such that _h = log(n) in the expected case_. The logarithmic growth ensures that the average search, insertion, and deletion operations remain efficient, typically with expected _O(log(n))_ time complexity.

William Pugh's <a href="https://15721.courses.cs.cmu.edu/spring2018/papers/08-oltpindexes1/pugh-skiplists-cacm1990.pdf" target="_blank" rel="noopener noreferrer">original paper</a> from 1990 conveniently spells out random level, search, insert, and remove operations as pseudocode that is used to guide this module's design. Note that the pseudocode is modified from the original paper to fit the notation convention present in this module (that is, CLRS-style with ASCII characters), but is otherwise unchanged from the original paper.

## The Search Algorithm
The search algorithm as its presented in the original paper generalizes some _public-facing operation_ for list search which returns either the node representing the `value` associated with a `search_key` or a failure/nil/None value to indicate that the `search_key` is not in the list.
```text
0    Search(list, search_key)
1      x = list.header
2      // loop invariant: x.key < search_key
3      for i = list.level downto 1 do
4        while x.forward[i].key < search_key do
5          x = x.forward[i]
6      // x.key < search_key <= x.forward[1].key
7      x = x.forward[1]
8      if x.key == search_key then return x.value
9        else return failure
```
It makes sense to split this algorithm into two different pieces, roughly separated at line 5. In this implementation, the first part represents a private search operation `skip_search(e)` that returns a position that is strictly < `search_key`. This sub-routine is then re-used by the `get(e)`, `contains(e)`, `insert(e)`, and `remove(e)` operations. If the list is empty, `skip_search(e)` returns `0`. An empty list contains a single sentinel node, so there is _always_ a previous node to insert a value, even if its the sentinel.

The second part of the algorithm simply represents a forward iteration and an equality check with the supplied `search_key`. This second phase is represented in a public `get(e)` operation that returns the value associated with the `search_key`, if it exists in the list. The equality check is crucial to determine whether the "next" node is actually the one being searched for.

## Insertion & Removal Algorithms
The insertion and deletion algorithms re-use much of the search algorithm's first phase, so they can be abstracted into [SkipList::skip_search] operations which return the node that is strictly smaller than the "search_key", which in this case is a new entry. The rest of the algorithm creates a new `SkipNode`, generates the "tower" len with a random number generator, populates the next node array for each level, and sets a singular previous node position.
The
```text
 0    Insert(list, search_key, newValue)
 1      local update[1..MaxLevel]
 2      x = list.header
 3      for i = list.level downto 1 do
 4        while x.forward[i].key < search_key do
 5          x = x.forward[i]
 6        // x.key < search_key <= x.forward[i].key
 7        update[i] = x

 8      x = x.forward[1]
 9      if x.key = search_key then x.value = newValue
10      else
11        lvl = randomLevel()
12        if lvl > list.level then
13          for i = list.level + 1 to lvl do
14            update[i] = list.header
15          list.level = lvl
16        x = makeNode(lvl, search_key, value)
17        for i = 1 to level do
18          x.forward[i] = update[i].forward[i]
19          update[i].forward[i] = x
```

This structure uses a contiguous backing structure instead of stable pointers/Position objects. As a result the list cannot strictly maintain the original design's asymptotics. The major advantage of linked lists is _O(1)_ node insertion/removal if a handle exists to the node. Contiguous lists generally require either _O(n)_ moves for insertion/removal of arbitrary elements. However, there are two options to deal with this; either use a [Vec::swap_remove] operation for _O(1)_ removals without wasting space, or using a free list to identify and fill holes after removal. For simplicity, this structure uses the first approach, meaning that indexes are _not_ stable, and as such are not surfaced in the public API. This design keeps the space requirements in check, but changes the canonical _O(log(n))_ removal time to _O(n * height)_, which is _O(n * log(n)) expected_, and _O(n^2)_ worst case (even though the list's height is technically capped).

Pugh's original removal algorithm (which is altered slightly in this implementation):
```text
 0    Delete(list, search_key)
 1      local update[1..MaxLevel]
 2      x = list.header
 3      for i = list.level downto 1 do
 4        while x.forward[i].key < search_key do
 5          x = x.forward[i]
 6        update[i] = x
 7      x = x.forward[1]
 8      if x.key = search_key then
 9        for i = 1 to list.level do
10          if update[i].forward[i] != x then break
11          update[i].forward[i] = x.forward[i]
12        free(x)
13        while list.level > 1 and
14          list.header.forward[list.level] == NIL do
15          list.level = list.level – 1
```
The `remove(e)` as it exists in this module:
```text
 0    Delete(list, searchKey)

 1      local update[0..MaxLevel] = FindPredecessors(list, searchKey)
 2      target = update[0].forward[0]

 3      // Early return for elements not in the list
 4      if target = NIL or target.key != searchKey then
 5        return failure
 6      last = list.nodes.last

 7      // unlink from skip structure
 8      for i = 0 to list.level - 1 do
 9        if update[i].forward[i] = target then
10          update[i].forward[i] = target.forward[i]

11      if target.forward[0] != NIL then
12        target.forward[0].prev = target.prev

13      removed = swap_remove(list.nodes, target)

14      // fix relocated node (if any)
15      if target < list.nodes.length then

16      for each node in list.nodes do
17        replace all forward pointers = last with target

18      if node at target has forward[0] != NIL then
19        forward[0].prev = target

20      if node at target.prev = last then
21        node.prev = target

22      while list.level > 1 and list.header.forward[list.level - 1] = NIL do
23        list.level -= 1

24      return removed.value
```

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

After inserting `['a', 'c', 'e', 'd', 'b', 'i', 'g', 'h', 'f']`, the list's `SkipNodes` might contain the following towers.
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
Note that its always possible to tell the last item in the list because its tower is empty. This makes sense, because the last element within the sorted arrangement can only point to `None`. As you can see by the index notation on the left-hand side of the table, the backing structure retains the insertion order; the backing structure remains unsorted.

The structure simply appends elements to the backing structure, so when printed the list retains its insertion order, not its sorted arrangement. As a result, the towers appear to contain rather nonsensical values. However, if you follow the indexes from the `HEAD` node, and re-arrange the nodes into _lexicographically sorted order_, which is what the navigational algorithms in the skiplist achieve, you get the following towers.
```text
HEAD[0]: [1, 2, 9, 7]
a[1]: [5]
b[5]: [2]
c[2]: [4, 4]
d[4]: [3, 9]
e[3]: [9]
f[9]: [7, 7, 7]
g[7]: [8, 6, 6]
h[8]: [6]
i[6]: []
```

When you rotate the mapping 90 degrees you can start to visualize the skip list layers as logically linked lists formed by "next" element indexes.
```text
L3: [ g[7] ] -> None
L2: [ f[9] ] -> [ g[7] ] -> [ i[6] ] -> None
L1: [ c[2] ] -> [ d[4] ] -> [ f[9] ] -> [ g[7] ] -> [ i[6] ] -> None
L0: [ a[1] ] -> [ b[5] ] -> [ c[2] ] -> [ d[4] ] -> [ e[3] ] -> [ f[9] ] -> [ g[7] ] -> [ h[8] ] -> [ i[6] ] -> None
```
Finally, if you extend each "next" index reference to align with its sorted position within the list, a classical skip list diagram of towers emerges.
```text
L3: HEAD -------------------------------------------------------------------------> [ g[7] ] -------------------------> None
L2: HEAD -------------------------------------------------------------> [ f[9] ] -> [ g[7] ] -------------> [ i[6] ] -> None
L1: HEAD -------------------------> [ c[2] ] -> [ d[4] ] -------------> [ f[9] ] -> [ g[7] ] -------------> [ i[6] ] -> None
L0: HEAD -> [ a[1] ] -> [ b[5] ] -> [ c[2] ] -> [ d[4] ] -> [ e[3] ] -> [ f[9] ] -> [ g[7] ] -> [ h[8] ] -> [ i[6] ] -> None
```

# Example code
```rust
    let mut list = dsa_rust::sequences::indexed_skip_list::SkipList::<char>::new();

    // An unsorted list of values and a sorted version to compare against
    let values = ['a', 'c', 'e', 'd', 'b', 'i', 'g', 'h', 'f'];
    let sorted = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];

    // Inserts unsorted values into the skip list with a consuming iterator
    for e in values.into_iter() {
        list.insert(e)
    }

    // Illustrates that the list exists as a sorted invariant
    for (i, e) in list.iter().enumerate() {
        assert_eq!(e, &sorted[i]);
    }

    // Illustrates the Kth function in a 0-indexed list.
    // That is, e occupies the 2 index for insertion order,
    // but is the 4th element in the 0-indexed sorted arrangement.
    assert_eq!(list.get_kth(4).unwrap(), &'e');

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
//const MAX_HEIGHT: usize = 10;

#[derive(Clone, Debug)]
struct SkipNode<T> {
    value: Option<T>,                  // None for sentinel
    next: [Option<usize>; MAX_HEIGHT], // forward links
    prev: Option<usize>,               // back links at s0 for reverse iteration
                                       //height: usize                      // stores the node's "tower" height
}

pub struct SkipList<T> {
    nodes: Vec<SkipNode<T>>,
    height: usize,
}
impl<T: Ord> Default for SkipList<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Ord> SkipList<T> {
    /// Creates a new, empty SkipList.
    pub fn new() -> Self {
        let sentinel = SkipNode {
            value: None,
            next: [None; MAX_HEIGHT],
            prev: None,
        };

        Self {
            nodes: vec![sentinel],
            height: 1,
        }
    }

    /// Returns the number of elements in the list.
    pub fn len(&self) -> usize {
        // Even empty lists have a single HEAD node,
        // which does not count
        self.nodes.len() - 1
    }

    /// Wrapper for `len()` that returns a Boolean
    /// indicating whether the list is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.len() - 1 == 0
    }

    /// Returns the a reference to the entry associated with the search key
    /// if it exists in the list, otherwise returns `None` to indicate
    /// that the key is not in the list.
    ///
    /// Represents Pugh's canonical `Search` operation as described in the
    /// <a href="https://15721.courses.cs.cmu.edu/spring2018/papers/08-oltpindexes1/pugh-skiplists-cacm1990.pdf" target="_blank" rel="noopener noreferrer">original paper</a>.
    pub fn get<Q>(&self, key: &Q) -> Option<&T>
    where
        Q: Ord + ?Sized,
        T: Borrow<Q>,
    {
        //let idx = self.skip_search(key);
        let idx = self.find_predecessors(key)[0];
        let next = self.nodes[idx].next[0]?;
        let val = self.nodes[next].value.as_ref()?;

        (val.borrow() == key).then_some(val)
    }

    /// Returns a Boolean indicating whether the supplied search key
    /// exists in the list.
    ///
    /// Wrapper for the public `get()` operation, which itself wraps
    /// the private `skip_search()` operation.
    pub fn contains<Q>(&self, key: &Q) -> bool
    where
        Q: Ord + ?Sized,
        T: Borrow<Q>,
    {
        self.get(key).is_some()
    }

    /// Inserts a new entry into the skip list.
    ///
    /// Allows duplicates, where ordering is determined by insertion order
    /// such that the most recent duplicates come before older entries.
    pub fn insert(&mut self, entry: T) {
        // Insert(list, search_key, newValue)
        //   local update[1..MaxLevel]
        //   x = list.header
        //   for i = list.level downto 1 do
        //     while x.forward[i].key < search_key do
        //       x = x.forward[i]
        //     // x.key < search_key <= x.forward[i].key
        //     update[i] = x

        // Chooses a random tower height and resets the list height
        // if it is taller than the current list height
        let height = self.random_height();
        if height > self.height {
            self.height = height;
        }

        // find_predecessors returns an array of predecessor positions
        // at each level for the splice point where update[0] is the
        // entry in the base list strictly < entry
        let update = self.find_predecessors(&entry);
        let prev_idx = update[0];
        let new_index = self.nodes.len(); // Backing list insertion index
        let next_idx = self.nodes[prev_idx].next[0];

        self.nodes.push(SkipNode {
            value: Some(entry),
            next: [None; MAX_HEIGHT],
            prev: Some(prev_idx),
        });

        // Reset the previous and current entry's next and previous
        // positions, respectively
        // take() only yields the number of elements in update up to
        // the list's height providing a minimal number of loop iterations
        for (level, _) in update.iter().enumerate().take(height) {
            let prev_idx = update[level];
            self.nodes[new_index].next[level] = self.nodes[prev_idx].next[level];
            self.nodes[prev_idx].next[level] = Some(new_index);
        }

        // If there is a "next" node it must now point back to the new node
        if let Some(next_idx) = next_idx {
            self.nodes[next_idx].prev = Some(new_index);
        }
    }

    /// Removes and returns the value for a given key, if it exists in
    /// the list. Returns None if the key does not exist in the list.
    ///
    /// This function does not technically adhere to Pugh's original
    /// removal algorithm. It uses [Vec::swap_remove] for simplified
    /// backing list compaction with the side effect of re-ordering remaining
    /// elements. The resultant removal time is therefore _O(n * height)_,
    /// which is _O(n * log(n)) expected_, and _O(n^2)_ worst case.
    pub fn remove<Q>(&mut self, key: &Q) -> Option<T>
    where
        Q: Ord + ?Sized,
        T: Borrow<Q>,
    {
        //  Delete(list, search_key)
        //    local update[1..MaxLevel]
        //    x = list.header
        //    for i = list.level downto 1 do
        //      while x.forward[i].key < search_key do
        //        x = x.forward[i]
        //      update[i] = x
        //    x = x.forward[1]
        //    if x.key = search_key then
        //      for i = 1 to list.level do
        //        if update[i].forward[i] != x then break
        //        update[i].forward[i] = x.forward[i]
        //      free(x)
        //      while list.level > 1 and
        //        list.header.forward[list.level] == NIL do
        //        list.level = list.level – 1

        // Pre-fetch precessors for target removal node
        // Technically O(n) but O(log(n)) expected
        let mut update = self.find_predecessors(key);

        // Check if the target is in the list, if it is, return its index
        let target = match self.nodes[update[0]].next[0] {
            Some(idx)
                if self.nodes[idx]
                    .value
                    .as_ref()
                    .is_some_and(|v| v.borrow() == key) =>
            {
                idx
            }
            _ => return None,
        };

        // Find the last node in the backing structure
        let last_idx = self.nodes.len() - 1;

        // Remove the prev and next positions from adjacent nodes
        if let Some(next_idx) = self.nodes[target].next[0] {
            self.nodes[next_idx].prev = self.nodes[target].prev;
        }
        for (level, val) in update.iter_mut().enumerate().take(self.height) {
            if self.nodes[*val].next[level] == Some(target) {
                self.nodes[*val].next[level] = self.nodes[target].next[level];
            }
        }

        // Actual node removal
        let removed_node = self.nodes.swap_remove(target);

        // The hot loop:
        // Set next/prev positions for the node that just got swapped
        // into the hole left by the removal
        //
        // Looks quadratic with nested for loops, but realistically
        // only requires O(n * height) worst case, where height is
        // expected to be log(n), so realistically this is O(n * log(n)),
        // and is expected to perform more like O(n) for sparse towers.
        if target < self.nodes.len() {
            // Fix next positions
            for node in &mut self.nodes {
                for next in node.next.iter_mut().take(self.height) {
                    if *next == Some(last_idx) {
                        *next = Some(target);
                    }
                }
            }

            // Repair adjacent backward links after relocation
            if let Some(next_idx) = self.nodes[target].next[0] {
                self.nodes[next_idx].prev = Some(target);
            }

            // Repair relocated predecessor reference
            if self.nodes[target].prev == Some(last_idx) {
                self.nodes[target].prev = Some(target);
            }
        }

        // Reduce the list's height, in case the removed tower was tallest
        while self.height > 1 && self.nodes[0].next[self.height - 1].is_none() {
            self.height -= 1;
        }

        // Return just the entry, not the entire node
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
        // FIND FRONT
        let front = match range.start_bound() {
            Bound::Included(start) => self.nodes[self.find_predecessors(start)[0]].next[0],
            Bound::Excluded(start) => {
                let idx = self.nodes[self.find_predecessors(start)[0]].next[0];
                if let Some(i) = idx {
                    if self.nodes[i].value.as_ref().unwrap().borrow() == start {
                        self.nodes[i].next[0]
                    } else {
                        Some(i)
                    }
                } else {
                    None
                }
            }
            Bound::Unbounded => self.nodes[0].next[0],
        };

        // FIND BACK
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
                        if update[0] == 0 {
                            None
                        } else {
                            Some(update[0])
                        }
                    }
                } else if update[0] == 0 {
                    None
                } else {
                    Some(update[0])
                }
            }
            Bound::Excluded(end) => {
                let update = self.find_predecessors(end);
                if update[0] == 0 {
                    None
                } else {
                    Some(update[0])
                }
            }
            Bound::Unbounded => {
                // To find the absolute end, we find predecessors for a
                // "theoretically infinite" value
                // or simply walk the tallest tower to the end.
                let mut curr = 0;
                for level in (0..self.height).rev() {
                    while let Some(next_idx) = self.nodes[curr].next[level] {
                        curr = next_idx;
                    }
                }
                if curr == 0 {
                    None
                } else {
                    Some(curr)
                }
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

    /// Returns an iterator over borrowed values in the list.
    pub fn iter(&self) -> Iter<'_, T> {
        // Walk the express lanes to find the very last node in O(log n) time
        let mut tail = 0;
        for level in (0..self.height).rev() {
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

    // Uses the external crate rand to determine the height h
    // of a given tower which is always 1 <= h < MAX_HEIGHT
    // by performing a series of "coin flips".
    fn random_height(&self) -> usize {
        let mut level = 1;
        let mut rng = rand::rng();
        while level < MAX_HEIGHT && rng.random::<bool>() {
            level += 1;
        }
        level
    }

    // Represents the heart of the skip list. This function is used
    // by the `insert()`, `remove()`, `range()`, `locate()` (and by
    // proxy `contains()`) functions.
    //
    // Returns an array of integers representing entries for
    // each level in the list that are strictly less than the search
    // key at each level, where the 0th index represents the base list.
    // The operation appears to be _O(n^2)_, but due to the list's
    // structure only requires _O(n)_ worst case, with _O(log(n))_ expected time.
    //
    // Performs dual duty in regards to Pugh's original design.
    // Useful as a basic skip_search by capturing the base list position
    // of the entry strictly < search_key as the 0th array index in the
    // return, as well as providing a list of previous positions at the
    // splice/split point for insert and delete operations.
    fn find_predecessors<Q>(&self, key: &Q) -> [usize; MAX_HEIGHT]
    where
        Q: Ord + ?Sized,
        T: Borrow<Q>,
    {
        let mut update = [0usize; MAX_HEIGHT];
        let mut idx = 0;

        for level in (0..self.height).rev() {
            loop {
                match self.nodes[idx].next[level] {
                    None => break,
                    Some(next_idx) => {
                        let next_val = self.nodes[next_idx].value.as_ref().unwrap();
                        if next_val.borrow() >= key {
                            break;
                        }
                        idx = next_idx;
                    }
                }
            }
            // Record the node index in the update list before descending
            update[level] = idx;
        }
        update
    }

    /// Represents the heart of the skip list. This function is used
    /// by the `get()`, `insert()`, `remove()`, `range()`, and
    /// `contains()`) functions.
    ///
    /// Returns the index of the largest entry that is
    /// strictly less than the provided key.
    ///
    /// NOTE: UNUSED
    fn _skip_search<Q>(&self, key: &Q) -> usize
    where
        Q: Ord + ?Sized,
        T: Borrow<Q>,
    {
        // Empty lists have a single sentinel node
        if self.nodes.len() == 1 {
            return 0;
        };

        // p = s: Always start at the HEAD node (index 0)
        let mut pos = 0usize;

        // Iterate through levels from top to bottom (the vertical "below(p)" steps)
        for level in (0..self.height).rev() {
            // "Scan forward" horizontally across the current level
            while self.nodes[pos].next[level].is_some() {
                // Peek at the NEXT node index on the current logical linked list
                match self.nodes[pos].next[level] {
                    // Get the next node's value safely.
                    // If the search key is >= the forward node's value,
                    // advance the pos to that node. If the next node is
                    // either > key or None, break the loop, which
                    // moves to the next level.
                    Some(next_idx) => {
                        let next_val = self.nodes[next_idx].value.as_ref().unwrap().borrow();
                        if next_val < key {
                            pos = next_idx;
                        } else {
                            break; // Break and descend a level
                        }
                    }
                    None => {
                        break; // Break and descend a level
                    }
                }
            }
        }

        // If the position never advances beyond the sentinel,
        // the key is not either doesn't exist in the list,
        // or it belongs as the first element
        pos
    }

    /// Returns the traversal path of a search key by
    /// re-using the skip_search logic. The only difference is that
    /// this traversal records each index and returns the path.
    fn _traversal<Q>(&self, key: &Q) -> Vec<T>
    where
        Q: Ord + ?Sized,
        T: Borrow<Q> + Clone,
    {
        let mut vec = Vec::new();

        // p = s: Always start at the HEAD node (index 0)
        let mut pos = 0usize;

        // Iterate through levels from top to bottom (the vertical "below(p)" steps)
        for level in (0..self.height).rev() {
            // "Scan forward" horizontally across the current level
            while self.nodes[pos].next[level].is_some() {
                // Peek at the NEXT node index on the current logical linked list
                match self.nodes[pos].next[level] {
                    // Get the next node's value safely.
                    // If the search key is >= the forward node's value,
                    // advance the pos to that node. If the next node is
                    // either > key or None, break the loop, which
                    // moves to the next level.
                    Some(next_idx) => {
                        let next_val = self.nodes[next_idx].value.as_ref().unwrap().borrow();
                        if next_val < key {
                            vec.push(self.nodes[next_idx].value.clone().unwrap());
                            pos = next_idx;
                        } else {
                            break; // Break and descend a level
                        }
                    }
                    None => {
                        break; // Break and descend a level
                    }
                }
            }
        }
        vec
    }
}

pub struct Iter<'a, T> {
    list: &'a SkipList<T>,
    next: Option<usize>,
    prev: Option<usize>,
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
    println!("Insert elements in order: {:?}", &values);
    for e in values.into_iter() {
        list.insert(e)
    }
    println!(
        "LIST DIAGNOSTICS: \n\theight: {}\n\tlength: {}",
        list.height,
        list.nodes.len()
    );
    println!("Tower contents by insertion order, NOT sorted order:");
    for (i, e) in list.nodes.iter().enumerate() {
        // Collect only the Some values into a new Vec
        //let values: Vec<_> = e.next.iter().filter_map(|&x| x).collect();
        let values: Vec<_> = e.next.iter().collect();
        match e.value {
            Some(val) => println!("{val:>04} [{i}]: {values:?}"),
            None => println!("HEAD [{i}]: {values:?}"),
        }
        //if let Some(val) = e.value {
        //    let v = &val.to_string();
        //    println!("{v}[{i}]: {values:?}");
        //} else {
        //    println!("HEAD[0]: {values:?}");
        //}
    }
    println!();

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

    // Tests remove(e)
    // Removes the first element
    println!("Remove 'a'");
    list.remove(&'a');
    // Removes an arbitrary element
    println!("Remove 'e'");
    list.remove(&'e');
    // Removes the last element
    println!("Remove 'i'");
    list.remove(&'i');
    // List shrinks as expected
    assert_eq!(list.len(), 6);
    // List no longer contains elements
    assert!(!list.contains(&'e'));
    assert!(!list.contains(&'a'));
    // Cant remove what isn't there!
    assert!(list.remove(&'z').is_none());
    // Debug prints new layout
    print!("List updated values: [");
    for e in list.iter() {
        print!("{e:#?} ")
    }
    println!("]");

    // Debug prints the tower contents
    println!(
        "LIST DIAGNOSTICS: \n\theight: {}\n\tlength: {}",
        list.height,
        list.nodes.len()
    );
    println!("Tower contents by insertion order, NOT sorted order:");
    for (i, e) in list.nodes.iter().enumerate() {
        // Collect only the Some values into a new Vec
        //let values: Vec<_> = e.next.iter().filter_map(|&x| x).collect();
        let values: Vec<_> = e.next.iter().collect();
        match e.value {
            Some(val) => println!("{val:>04} [{i}]: {values:?}"),
            None => println!("HEAD [{i}]: {values:?}"),
        }
    }
    println!();

    // Tests skip_search(e), find_predecessors(e) and their dependencies:
    // contains(e), get(e), and find_val(e)
    //
    // An element in the list
    let node = 'h';
    assert_eq!(list._skip_search(&node), 6); // 6 is g which is < h
    assert_eq!(list.find_predecessors(&node)[0], 6); // 6 is g which is < h
    assert!(list.contains(&node));
    assert_eq!(list.get(&node).unwrap(), &'h');
    // An element at the beginning of the list
    let node = 'b';
    assert_eq!(list._skip_search(&node), 0);
    assert_eq!(list.find_predecessors(&node)[0], 0);
    assert!(list.contains(&node));
    assert_eq!(list.get(&node).unwrap(), &'b');
    // An element not in the list
    // 'j' is not in the list, but skip_search returns 6 because
    // thats the position that 'i' lives at due to insertion order,
    // and 'i' < 'j'
    let node = 'j';
    assert_eq!(list._skip_search(&node), 3); // 3 is h, the last entry
    assert_eq!(list.find_predecessors(&node)[0], 3); // 3 is h, the last entry
    assert!(!list.contains(&node));
    assert!(list.get(&node).is_none());
    // An element that was previously in the list, but removed
    let node = 'a';
    assert_eq!(list._skip_search(&node), 0); // belongs after HEAD
    assert_eq!(list.find_predecessors(&node)[0], 0); // belongs after HEAD
    assert!(!list.contains(&node));
    assert!(list.get(&node).is_none());

    // A bunch of random list mutations to ensure coherence
    list.insert('p');
    list.insert('u');
    list.insert('w');
    list.remove(&'p');
    list.insert('l');
    list.insert('m');
    list.remove(&'f');
    list.remove(&'o');
    list.insert('q');
    list.remove(&'m');
    list.insert('x');
    list.insert('z');

    // Visual component:
    // Combines contains(e) and prints traversal() as proof
    let trav = list._traversal(&'g');
    let con = list.contains(&'g');
    println!("Contains 'g': {con:?}");
    println!("Traversal: {trav:?}");
    let trav = list._traversal(&'j');
    let con = list.contains(&'j');
    println!("Contains 'j': {con:?}");
    println!("Traversal: {trav:?}");
    let trav = list._traversal(&'a');
    let con = list.contains(&'a');
    println!("Contains 'a': {con:?}");
    println!("Traversal: {trav:?}");

    // Tests traversal ordering
    let l2 = ['b', 'c', 'd', 'g', 'h', 'l', 'q', 'u', 'w', 'x', 'z'];
    for (val, i) in list.iter().zip(0..=5) {
        assert_eq!(val, &l2[i]);
    }
    // Visual confirmation of correct traversal
    print!("List values:\n   ");
    for e in list.iter() {
        print!("{e:#?} ")
    }
    println!();

    //panic!();
}

#[test]
// AI-written "stress" test
fn test_skip_list_removal_integrity() {
    // Assumes your SkipList has a standard New or Default implementation
    let mut list = SkipList::new();

    // 1. Insert a sequence of numbers.
    // This allows your natural random_height() generator to build up
    // a multi-level tower structure organically.
    let total_elements = 300;
    for i in 0..total_elements {
        list.insert(i);
    }

    // 2. Remove elements from the middle of the list.
    // This forces swap_remove to repeatedly pull the last element of the Vec
    // into the newly created holes, triggering your global repair loops.
    for i in (50..200).step_by(2) {
        list.remove(&i);
    }

    // 3. STRUCTURAL AUDIT
    // Scan every single surviving node's next pointers across every level.
    // If swap_remove left a stale index behind, it will point out-of-bounds.
    let current_len = list.nodes.len();
    for idx in 0..current_len {
        for level in 0..list.height {
            if let Some(next_idx) = list.nodes[idx].next[level] {
                // Assert 1: Out-of-bounds index protection
                assert!(
                    next_idx < current_len,
                    "CORRUPTION: Node at index {idx} on level {level} points to index {next_idx}, \
                     which is out-of-bounds for a Vec of length {current_len}!"
                );

                // Assert 2: Level 0 backlink symmetric validation
                if level == 0 {
                    assert_eq!(
                        list.nodes[next_idx].prev,
                        Some(idx),
                        "CORRUPTION: Link symmetry broken! Node {} points forward to {}, \
                         but {} points backward to {:?}",
                        idx,
                        next_idx,
                        next_idx,
                        list.nodes[next_idx].prev
                    );
                }
            }
        }
    }

    // 4. ALGORITHMIC AUDIT
    // Verify that every single element that wasn't deleted is still perfectly
    // searchable via skip_search. If a lane broke, skip_search will overshoot
    // or fail to track the element.
    for i in 0..total_elements {
        // Skip the elements we explicitly deleted
        if (50..200).contains(&i) && i % 2 == 0 {
            continue;
        }

        // Your skip_search returns a valid usize position
        let pos = list.find_predecessors(&i)[0];

        // Ensure the pos returned actually matches our search target
        // or points to its direct predecessor.
        if pos == 0 {
            // If it returned the sentinel, the first item in the list must be >= i
            if let Some(first_idx) = list.nodes[0].next[0] {
                let first_val = list.nodes[first_idx].value.as_ref().unwrap();
                assert!(first_val >= &i);
            }
        } else {
            let found_val = list.nodes[pos].value.as_ref().unwrap();
            assert!(
                found_val <= &i,
                "skip_search returned a node greater than the key!"
            );
        }
    }

    println!(
        "LIST DIAGNOSTICS: \n\theight: {}\n\tlength: {}",
        list.height,
        list.nodes.len()
    );
    println!("Tower contents by insertion order, NOT sorted order:");
    for (i, e) in list.nodes.iter().enumerate() {
        // Collect only the Some values into a new Vec
        //let values: Vec<_> = e.next.iter().filter_map(|&x| x).collect();
        let values: Vec<_> = e.next.iter().collect();
        match e.value {
            Some(val) => println!("{val:>04} [{i}]: {values:?}"),
            None => println!("HEAD [{i}]: {values:?}"),
        }
    }
    println!();
    //panic!()

    // Maximalist
    let array: [i32; 4] = [1i32, 2, 3, 4];
    let iterator: std::slice::Iter<'_, i32> = array.iter();
    let total: i32 = iterator.sum();
    assert_eq!(total, 10);

    // Minimalist
    assert_eq!([1, 2, 3, 4].iter().sum::<i32>(), 10);

    // Illustrative
    let array = [1, 2, 3, 4]; // Iterable object
    let iter = array.iter(); // Iterator
    let total: i32 = iter.sum(); // Iterator adapter
    assert_eq!(total, 10);

    // Also illustrative
    let array = [1, 2, 3, 4];
    let mut total = 0;
    for e in array.iter() {
        total += e;
    }
    assert_eq!(total, 10);

    // Baby bear; just right
    let array = [1, 2, 3, 4]; // Iterable object
    let total: i32 = array.iter().sum(); // Iterator and adapter
    assert_eq!(total, 10);
    assert_eq!(array, [1, 2, 3, 4]);
}
