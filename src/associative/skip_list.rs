// NOTE: This is a de-prioritized WIP and may be on ice for a while,
// please pardon the mess
#![allow(warnings, clippy::all)]

/*! A naive attempt at implementing a skip list

# About
The idea of a skip list is to implement sorted structures with _expected O(log(n))_ time complexity for search, insert, and removal operations. This provides a significant advantage over sorted array-based lists, which have _worst-case O(n)_ removal (average _O(n/2)_) temporal performance. Skip lists are also simpler than self-balancing tree structures and provide an easier and finer-grained control when adapting these structures for concurrent operations. There is a reason Java's `concurrentSkipListMap` is so popular.

This structure specifically implements the [SkipListMap<K, V>] structure which provides a map implementation, as the name suggests. See the [SkipListSet<T>] module for a set implementation.

# Design
At it's root is the logic behind the [doubly-linked list](crate::sequences::doubly_linked_list) which squarely places this list into "advanced Rust" territory. Later, when we implement concurrency, this will propel the skip list map to "expert Rust" territory.

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

For example, at level 2 you could traverse the list as pointers to [2, 7, 9]. The distinction here is that all of the node data is stored in one location, with only pointers being duplicated and populating tower lists.

# Example code
```rust
```
*/
use std::fmt::Debug;

// Creates a raw pointer to some Node
type Link<T> = Option<*mut SkipNode<T>>;

#[derive(Debug)]
struct SkipNode<T>
where
    T: Ord + PartialEq,
{
    data: Option<T>, // Each allocated node MUST contain data, only sentinels can be None
    level: usize,    // The current node's level
    tower: Vec<Link<T>>, // A list of "next" nodes for each level in the node's tower
    prev: Link<T>,   // The prev node in S0
    next: Link<T>,   // The next node in S0
}
impl<T> SkipNode<T>
where
    T: Debug + Ord,
{
    fn new_sen() -> Self {
        SkipNode {
            data: None,
            level: 0,
            tower: Vec::new(),
            prev: None,
            next: None,
        }
    }

    /// Returns `true` if the data for the given node is None, which
    /// marks it as a sentinel node.
    fn is_sen(&self) -> bool {
        self.data.is_none()
    }
}

#[derive(Debug)]
pub struct SkipList<T>
where
    T: Ord + PartialEq,
{
    head: Link<T>,
    height: usize, // 0-indexed dynamic max height value
    size: usize,   // Number of Some nodes in the list (no ghosts! 👻)
}
impl<T> SkipList<T>
where
    T: Debug + PartialEq + Ord,
{
    /// Creates a new `SkipList` with a single sentinel node.
    pub fn new() -> Self {
        SkipList {
            //data: Some(Box::into_raw(Box::new(SkipNode::new_sen()))),
            head: None, // Sentinel node is always None
            height: 0,
            size: 0,
        }
    }

    /// Returns true if there is only an empty sentinel node in the list
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Returns a link to the node that is guaranteed to be <= the given value, allowing placement
    /// into the "next" node slot.
    ///
    /// WARNING: Unimplemented
    fn skip_search(&self, element: &T) -> Option<Link<T>> {
        // Case 1) The list is empty
        //if self.is_empty() {
        //    return None;
        //}

        // Case 2) The list contains elements

        // Search only if there are real nodes in the list
        if let Some(node) = self.head {
            // Start at the highest level of the head node
            // SAFETY: The list always has a head node with a Vec of Links
            let link = unsafe { (&(*node).tower)[self.height] };

            // Follow the link to the first node
            if let Some(value) = link {
                //
            }

            let mut node_data = unsafe { (&(*node).tower)[self.height].unwrap().as_ref().unwrap() };

            // The actual skip search algorithm:
            let mut result = std::cmp::Ordering::Less;

            // let mut level = self.height;  // Start at the structure's max height
            //
            // while level >= 0 {
            //     // 1) Scan forward at current level
            //     while let Some(next) = node.next[level] {
            //         if next.key < k {
            //             node = next;
            //         } else {
            //             break;
            //         }
            //     }
            //
            //     // 2) Drop down a level
            //     if level == 0 {
            //         break;
            //     } else {
            //         level -= 1;
            //     }
            // }
            //
            // return node;  // Node such that node.key <= k

            Some(self.head) // Algorithmic placeholder
        } else {
            None
        }
    }

    pub fn insert(&mut self, entry: T) {
        // Find insertion point
        let insert_after = self.skip_search(&entry);

        // Allocate
        let node = Box::new(entry);
        let node_ptr = Box::into_raw(node);

        // Build new entry tower
        // let n = Self::tower_roll();

        // Identify all
        //
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn first() {
        let sk: SkipList<usize> = SkipList::new();
    }
}
