/*! An arena-allocated (min) binary heap implementation

# About
This structure represents a min heap (by default). This means that elements with the smallest ordered values sit at the top of the heap (and get popped first). E.g. an element with priorty 6 gets popped before an element with the priority 9. You can implement [std::cmp::Reverse] to construct max heaps, though it does introduce wrapping which prevents seamless type compatability or interface consistency.

Push and pop operations are guaranteed to run in `O(log(n))` time, but sorting is amortized to `O(n * log(n))` time, which is exacerbated by reverse-sorted inputs.

# Design
A `Vec`-backed structure is particularly appropriate for heap implementation due to the complete binary tree invariant. Complete binary trees allow you to infer a lot of information in an indexed structure that you would not get with a linked structure. 

- With a complete tree you know that the maximum depth (height) of the tree is always always ⌊log2(`n`)⌋ where `n` is the number of elements in the heap. 

- Finding an insert point is always `O(1)` in an indexed structure with a known length. Finding an insert point in a linked backing structure requires `O(log(n))` traversal. 

- Element types (parent, left child, and right child) can be determined by a mathematical relationship in the backing structure. For example, for index `i`, if `heap[i] == root`, then `i + 1 == root.left`, and `i + 2 == root.right`. This can be extended to any element in the heap such that left children are always `2i + 1`, and right children are always `2i + 2`, and parents are always `(i - 1) / 2`. 

- You also know that indexes in a heap with backing structure `list` contain `(list.len() / 2)..list.len()` leaf nodes, and all indexes `0..(list.len() / 2)` are parent nodes. This, of course, relies on _integer division_ (division in which all remainders are truncated/discarded, not rounded).

For a visual example, consider the following layout: 
| Index | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9  |  10 | 11 |
|-------|---|---|---|---|---|---|---|---|---|----|-----|----|
| Value | A | B | C | D | E | F | G | H | I | J  |  K  | L  |

This produes the following heap structure:
```text
             A
         /       \
       B           C
     /   \        / \
    D     E      F   G
   / \   / \    /   
  H   I J   K  L
```
You can clearly see that nodes at indexes 6 through 11 are leaf nodes, corresponding to the range
`(list.len() / 2)..list.len()` when `list.len() == 12`. Conversely, indexes 0 through 5 are parent nodes, corresponding to the range `0..(list.len() / 2)`.

You can also see that all indexes of the form `2i + 1` are left children. E.g. indexes [1, 3, 5, 7, 9, 11] are all left children. The parent of a node at index `i` is always at `(i - 1) / 2`, which produces the mapping: 

```text
    M = { (0,3), (1,2), (2,2), (3,2), (4,2), (5,1) }
```

# Example

```rust
    use dsa_rust::trees::bin_heap::BinHeap;

    // Defines an arbitrary entry object
    // NOTE: Clone is only necessary for from_slice() and list reuse
    #[derive(Clone, Eq, PartialEq, PartialOrd)]
    pub struct Job<'a> {
        pub priority: usize,
        pub title: &'a str,
    }

    // Heaps require total ordering, and BinHeap places an Ord trait bound on T.
    // This implementation guarantees ordering by the priority field only.
    // Although PartialOrd is derived, it is unused in BinHeap because all comparisons
    // are made using the Ord implementation.
    use std::cmp::Ordering;
    impl Ord for Job<'_> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.priority.cmp(&other.priority)
        }
    }

    let list = vec![
        Job {
            priority: 9,
            title: "Dichael",
        },
        Job {
            priority: 13,
            title: "Damiel",
        },
        Job {
            priority: 8,
            title: "Sleve",
        },
        Job {
            priority: 6,
            title: "Peter",
        },
        Job {
            priority: 14,
            title: "Flank",
        },
        Job {
            priority: 11,
            title: "Cork",
        },
        Job {
            priority: 12,
            title: "Bobson",
        },
    ];

    // Heapify options
    //////////////////

    // Option 1: from_slice() in O(n) time
    let mut heap = BinHeap::from_slice(&list);
    assert_eq!(heap.pop().unwrap().title, "Peter");
    assert_eq!(heap.pop().unwrap().title, "Sleve");

    // Option 2: from_vec() in O(n) time, clones purely for 
    // list reuse in this test
    let mut heap = BinHeap::from_vec(list.clone());
    assert_eq!(heap.pop().unwrap().title, "Peter");
    assert_eq!(heap.pop().unwrap().title, "Sleve");

    // Examples that use Reverse to construct a max heap
    use std::cmp::Reverse;

    // Option 1: O(n) reverse heapify, clones purely for list reuse
    let reversed_vec: Vec<Reverse<_>> = list.clone().into_iter().map(Reverse).collect();
    let mut heap = BinHeap::from_vec(reversed_vec);
    assert_eq!(heap.pop().unwrap().0.title, "Flank");
    assert_eq!(heap.pop().unwrap().0.title, "Damiel");

    // Option 2: O(n * log(n)) incremental reverse heap construction
    let mut heap = BinHeap::new();
    for e in list {
        heap.push(Reverse(e))
    }
    assert_eq!(heap.pop().unwrap().0.title, "Flank");
    assert_eq!(heap.pop().unwrap().0.title, "Damiel");

    // Dirty lil heap sort
    //////////////////////

    let list = [1, 5, 7, 23, 45, 3, 2, 17, 56, 9, 21];

    // Creates a heap in O(n) time
    let mut heap = BinHeap::from_slice(&list);

    // Creates a sorted list in O(n * log(n)) time
    let mut sorted_list: Vec<i32> = Vec::new();
    for _ in 0..list.len() {
        sorted_list.push(heap.pop().expect("oh noez"))
    }

    assert_eq!(sorted_list, [1, 2, 3, 5, 7, 9, 17, 21, 23, 45, 56]);


```

*/

/// # About
/// It's `Vec` with a brand new dress.
///
/// See the [module-level documentation](crate::trees::bin_heap) for more information.
#[derive(Debug)]
pub struct BinHeap<T>
where
    T: Ord,
{
    arena: Vec<T>,
    size: usize,
}
impl<T> Default for BinHeap<T>
where
    T: Ord
 {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> BinHeap<T>
where
    T: Ord
{
    // PUBLIC API
    /////////////

    /// Creates a new, zero-sized `BinHeap`.
    pub fn new() -> Self {
        BinHeap {
            arena: Vec::new(),
            size: 0,
        }
    }

    /// Creates a new `BinHeap` that pre-allocates to the given capacity.
    pub fn new_with_capacity(capacity: usize) -> Self {
        BinHeap {
            arena: Vec::with_capacity(capacity),
            size: 0,
        }
    }

    /// A "heapify" operation that creates a `BinHeap` from any slice of entries 
    /// in `O(n)` time. The entries must be `Clone`.
    pub fn from_slice(list: &[T]) -> Self 
        where T: Clone {
        let mut heap = BinHeap::new();
        heap.arena = Vec::from(list);
        let start = (list.len() / 2).saturating_sub(1);
        for i in (0..=start).rev() {
            heap.sift_down(i);
        }
        heap
    }

    /// A "heapify" operation that creates a `BinHeap` from a `Vec<T>` in `O(n)` 
    /// time. `T` does not need to be `Clone`.
    pub fn from_vec(list: Vec<T>) -> Self {
        let size = list.len();
        let mut heap = BinHeap { arena: list, size };
        let start = (size / 2).saturating_sub(1);
        for i in (0..=start).rev() {
            heap.sift_down(i);
        }
        heap
    }

    /// Returns the number of elements in the heap.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Adds an element to the heap in `O(log(n))` time.
    pub fn push(&mut self, data: T) {
        // 1) Create and insert a Node into the array in a way that maintains the
        // complete binary tree structure
        self.arena.push(data);

        // 2) Sift/bubble/percolate the heap to maintain order
        // in O(log(n)) time
        self.sift_up(self.arena.len() - 1);
    }

    /// Returns an immutable reference to the data at the top of the heap 
    /// in `O(1)` time.
    pub fn peek(&self) -> Option<&T> {
        if self.arena.is_empty() {
            None
        } else {
            Some(&self.arena[0])
        }
    }

    /// Returns and removes the element at the top of the heap in `O(log(n))` time.
    pub fn pop(&mut self) -> Option<T> {
        if !self.arena.is_empty() {
            let node = self.arena.swap_remove(0); // O(1)
            self.sift_down(0); // O(log(n))
            Some(node)
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
            if self.arena[index] < self.arena[parent_index] {
                self.arena.swap(index, parent_index);
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
            if left_child < self.arena.len()
                && self.arena[left_child] < self.arena[target] {
                target = left_child;
            }
            if right_child < self.arena.len()
                && self.arena[right_child] < self.arena[target] { 
                target = right_child;
            }
            
            // 2) If the target is not equal to the index, do the swap operation 
            // and continue sifting, otherwise the heap property is true
            if target != index {
                self.arena.swap(index, target);
                index = target; // Move down
            } else {
                break; // No-op, heap is heapy 🍃🧘
            }
        }
    }

    /// Sorts a mutable slice into ascending order in place via (max) heap operations in `O(n * log(n))` time.
    ///
    /// ```rust
    /// use dsa_rust::trees::bin_heap::BinHeap;
    /// let mut v = Vec::from([8, 6, 7, 5, 3, 0, 9]);
    /// BinHeap::heap_sort(&mut v);
    /// assert_eq!(v, [0, 3, 5, 6, 7, 8, 9]);
    /// ```
    pub fn heap_sort(list: &mut [T]) 
        where T: Ord {
        let len = list.len();
    
        // Transform the list into a heap in O(n) time
        for i in (0..len / 2).rev() {
            Self::generic_sift_down(list, i, len); // O(log(n))
        }
    
        // Sort the heap by sifting/sorting in place in O(n * log(n))
        for end in (1..len).rev() {
            list.swap(0, end);
            Self::generic_sift_down(list, 0, end); // O(log(n))
        }
    }
    
    /// Essentially the as sift_down but takes a list (not a heap as self), and 
    /// performs max heap sifting instead of default min heap sifting.
    fn generic_sift_down(list: &mut [T], mut root: usize, end: usize) {
        loop {
            let left = 2 * root + 1;
            let right = 2 * root + 2;
            let mut largest = root;
    
            if left < end && list[left] > list[largest] {
                largest = left;
            }
            if right < end && list[right] > list[largest] {
                largest = right;
            }
    
            if largest != root {
                list.swap(root, largest);
                root = largest;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn min_heap_test() {

        // Defines an arbitrary entry object, 
        // NOTE: Clone is only necessary for from_slice() usage
        //#[derive(Clone, Debug, Eq, PartialEq)]
        #[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
        pub struct Job<'a> {
            pub priority: usize,
            pub title: &'a str,
        }

        // Heaps require total ordering, so BinHeap places an Ord trait bound on T.
        // This implementation guarantees ordering by the priority field only.
        // Although PartialOrd is derived, it is unused in BinHeap because all comparisons
        // are made using the Ord implementation.
        use std::cmp::Ordering;
        impl Ord for Job<'_> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.priority.cmp(&other.priority)
            }
        }

        // Heaps require total order, so a custom PartialOrd impl is unnecessary
        //impl PartialOrd for Job<'_> {
        //    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        //        Some(self.cmp(other))
        //    }
        //}

        let list = vec![
            Job {
                priority: 9,
                title: "Dichael",
            },
            Job {
                priority: 13,
                title: "Damiel",
            },
            Job {
                priority: 8,
                title: "Sleve",
            },
            Job {
                priority: 6,
                title: "Peter",
            },
            Job {
                priority: 14,
                title: "Flank",
            },
            Job {
                priority: 11,
                title: "Cork",
            },
            Job {
                priority: 12,
                title: "Bobson",
            },
        ];

        // Heapify options!
        ///////////////////

        // Option 1: from_slice()
        let mut heap = BinHeap::from_slice(&list);
        assert_eq!(heap.pop().unwrap().title, "Peter");
        assert_eq!(heap.pop().unwrap().title, "Sleve");
        assert_eq!(heap.pop().unwrap().title, "Dichael");
        assert_eq!(heap.pop().unwrap().title, "Cork");
        assert_eq!(heap.peek().unwrap().title, "Bobson");
        assert_eq!(heap.pop().unwrap().title, "Bobson");
        assert_eq!(heap.pop().unwrap().title, "Damiel");
        assert_eq!(heap.peek().unwrap().title, "Flank");

        // Option 2: from_vec(), clones purely for list reuse
        let mut heap = BinHeap::from_vec(list.clone());
        assert_eq!(heap.pop().unwrap().title, "Peter");
        assert_eq!(heap.pop().unwrap().title, "Sleve");
        assert_eq!(heap.pop().unwrap().title, "Dichael");
        assert_eq!(heap.peek().unwrap().title, "Cork");
        assert_eq!(heap.pop().unwrap().title, "Cork");
        assert_eq!(heap.pop().unwrap().title, "Bobson");
        assert_eq!(heap.pop().unwrap().title, "Damiel");
        assert_eq!(heap.peek().unwrap().title, "Flank");

        // Examples that use Reverse to construct a max heap
        use std::cmp::Reverse;

        // Option 1: O(n) reverse heapify, clones purely for list reuse
        let reversed_vec: Vec<Reverse<_>> = list.clone().into_iter().map(Reverse).collect();
        let mut heap = BinHeap::from_vec(reversed_vec);
        assert_eq!(heap.pop().unwrap().0.title, "Flank");
        assert_eq!(heap.pop().unwrap().0.title, "Damiel");
        assert_eq!(heap.pop().unwrap().0.title, "Bobson");
        assert_eq!(heap.pop().unwrap().0.title, "Cork");
        assert_eq!(heap.peek().unwrap().0.title, "Dichael");
        assert_eq!(heap.pop().unwrap().0.title, "Dichael");
        assert_eq!(heap.pop().unwrap().0.title, "Sleve");
        assert_eq!(heap.peek().unwrap().0.title, "Peter");

        // Option 2: O(n * log(n)) incremental reverse heap construction
        let mut heap = BinHeap::new();
        for e in list {
            heap.push(Reverse(e))
        }
        assert_eq!(heap.pop().unwrap().0.title, "Flank");
        assert_eq!(heap.pop().unwrap().0.title, "Damiel");
        assert_eq!(heap.pop().unwrap().0.title, "Bobson");
        assert_eq!(heap.pop().unwrap().0.title, "Cork");
        assert_eq!(heap.peek().unwrap().0.title, "Dichael");
        assert_eq!(heap.pop().unwrap().0.title, "Dichael");
        assert_eq!(heap.pop().unwrap().0.title, "Sleve");
        assert_eq!(heap.peek().unwrap().0.title, "Peter");

    }

    #[test]
    // Tests two approaches to heap sort:
    // 1) Creating a separate binary heap buffer, pushing values, and popping 
    //    them back to a third sorted list.
    // 2) Treating the backing list as the buffer for in-place heap creation
    //    and sorting via pop & swap loop.
    fn heap_sort() {
        let mut list = [1, 5, 7, 23, 45, 3, 2, 17, 56, 9, 21];
        let sorted = [1, 2, 3, 5, 7, 9, 17, 21, 23, 45, 56];

        // 1) Sorting via heap buffer (secondary storage)
        // Creates a heap (buffer) in O(n) time from borrowed values
        let mut heap = BinHeap::from_slice(&list);
        // Pops the buffered heap values to a (secondary) sorted list in O(n * log(n)) time
        let mut sorted_list: Vec<i32> = Vec::new();
        for _ in 0..list.len() {
            sorted_list.push(heap.pop().expect("uh oh"))
        }
        assert_eq!(sorted_list, sorted);

        // 2) Sorting in place
        BinHeap::heap_sort(&mut list);
        assert_eq!(list, sorted);

        let mut v = Vec::from([6, 5, 8, 3]);
        BinHeap::heap_sort(&mut v);
        assert_eq!(v, [3, 5, 6, 8]);

    }

}
