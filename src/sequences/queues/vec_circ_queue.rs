/*! A circular Vec-based queue

# About
This simple, safe, Vec-based circular queue is mostly just a fun experiment, but can be used for situations in which you need a fixed-sized buffer with FIFO logic. The circular queue can be used to provide a solution to the [Josephus problem](https://en.wikipedia.org/wiki/Josephus_problem).

# Design
Rust takes its types (and its type safety) seriously. Array's must have a known size at compile time, forcing any array-backed structure's size hard coded. `[T; N]`

This example illustrates the circular queue logic. The example provides a queue postcondition for enqueue/dequeue operations to illustrate the state of the list after each operation. Remember that `enqueue()` adds to the _back_, and `dequeue()` removes from the _front_ of the queue.
```rust

   use dsa_rust::sequences::queues::vec_circ_queue::CircularQueue;

   // The queue is a fixed size
   let mut q: CircularQueue<char> = CircularQueue::new(3);

   // enqueue/dequeue return Result because adding/removing on a
   // full/empty queue is an error
   // Postcondition
   //
   //  <-> a <-> b <-> c <->
   //      ^           ^
   //   front        back
   q.enqueue('a');
   q.enqueue('b');
   q.enqueue('c');

   // The queue is at capacity, and the queue hasn't shifted
   assert_eq!(q.peek().unwrap(), &'a');
   assert_eq!(q.size(), q.capacity());
   assert_eq!(q.front(), 0);
   assert_eq!(q.back(), 2);

   // The queue cannot take additional elements
   assert!(q.enqueue_with_check('d').is_err());

   // Remove the head node and check changes:
   // Postcondition:
   // - There is only one item in the queue
   // - The front and back of the queue point to the same index
   // Postcondition
   //
   //  <-> None <-> b <-> c <->
   //               ^     ^
   //            front  back
   assert_eq!(q.dequeue().unwrap(), 'a');

   // Postcondition
   //
   //  <-> None <-> Non <-> c <->
   //                       ^
   //                  front/back
   assert_eq!(q.dequeue().unwrap(), 'b'); // queue: 0:None, 1:None, 2:c (fr/ba)
   assert_eq!(2, q.capacity() - q.size()); // Remaining capacity
   assert_eq!(q.front(), 2);
   assert_eq!(q.back(), 2);
   assert_eq!(q.size(), 1);

   // Adding new items wraps the queue, hence circular queue
   // Postcondition
   //
   //  <-> d <-> None <-> c <->
   //      ^              ^
   //    back           front
   q.enqueue('d');

   // Postcondition
   //
   //  <-> d <-> e <-> c <->
   //            ^     ^
   //          back  front
   q.enqueue('e');
   assert_eq!(q.front(), 2);
   assert_eq!(q.back(), 1);
   assert_eq!(q.size(), 3);

   // Removes two more elements and checks that there is just one element left
   // Postcondition
   //
   //  <-> d <-> e <-> None <->
   //      ^     ^
   //    front  back
   assert_eq!(q.dequeue().unwrap(), 'c');

   // Postcondition
   //
   //  <-> None <-> e <-> None <->
   //               ^
   //          front/back
   assert_eq!(q.dequeue().unwrap(), 'd');
   assert_eq!(2, q.capacity() - q.size()); // Remaining capacity
   assert_eq!(q.front(), 1);
   assert_eq!(q.back(), 1);
   assert_eq!(q.size(), 1);
   assert_eq!(q.peek().unwrap(), &'e');
```
*/

/// # About
/// All functions operate in `O(1)` time unless otherwise noted.
///
/// See the [module-level documentation](`crate::lists::queues::vec_circ_queue`) for more information.
pub struct CircularQueue<T> {
    data: Vec<Option<T>>, // Store elements as `Option` to allow reusing slots
    front: usize,
    back: usize,
    size: usize,
    capacity: usize,
}
impl<T> CircularQueue<T> {
    /// Creates a queue that contains (at least) `capacity` number of elements,
    /// and initializes all positions to `None` in `O(n)` time.
    pub fn new(capacity: usize) -> CircularQueue<T> {
        let mut data = Vec::with_capacity(capacity);
        // Fills the vec with None
        //data.resize_with(capacity, Default::default); // Clean and generic
        //data.resize_with(capacity, || None); // More explicit and slightly more efficient
        // Somehow the fastest initializiation option
        for _ in 0..capacity {
            data.push(None)
        }
        CircularQueue {
            data,
            front: 0,
            back: 0,
            size: 0,
            capacity,
        }
    }

    /// Returns the index representing the front of the queue.
    pub fn front(&self) -> usize {
        self.front
    }

    /// Returns the index representing the back of the queue.
    pub fn back(&self) -> usize {
        self.back
    }

    /// Returns the number of elements in the queue.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the total capacity of the queue.
    ///
    /// This is the maximum number of elements the queue can hold,
    /// determined at instantiation. It does not reflect the number of
    /// currently available slots. To find the number of free slots,
    /// subtract the current size from the capacity.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns a reference to the front of the queue.
    pub fn peek(&self) -> Option<&T> {
        self.data[self.front].as_ref()
    }

    /// Adds an element to the back of the queue. This operation has no size or
    /// capacity checks for efficiency and performance, may overwrite existing
    /// slots. Use the `enqueue_with_check()` operation to prevent data overwrites
    /// at the slight performance cost of an additional runtime check.
    pub fn enqueue(&mut self, item: T) {
        // Calculates the next available positionm, writes to it, and increases size
        self.back = (self.front + self.size) % self.capacity;
        self.data[self.back] = Some(item);
        self.size += 1;
    }

    /// Adds an element to the back of the queue. This operation checks that
    /// there is sufficient queue capacity to add an element and errors if you
    /// attempt to add more elements than the queue's capacity. The
    /// `enqueue()` operation is slightly more performanant but does not
    /// automatically prevent overwrites.
    pub fn enqueue_with_check(&mut self, item: T) -> Result<(), &str> {
        // Ensures that the queue cannot take more elements than its capacity
        if self.size == self.capacity {
            return Err("Queue is full");
        }
        // Calculates the next available positionm, writes to it, and increases size
        self.back = (self.front + self.size) % self.capacity;
        self.data[self.back] = Some(item);
        self.size += 1;
        Ok(())
    }

    /// Removes and returns the front element of the queue, if an element exists.
    pub fn dequeue(&mut self) -> Option<T> {
        // Checks if queue is empty and returns proper None
        if self.size == 0 {
            return None;
        }
        // Otherwise take() the value from the front, leaving None in its place
        let item = self.data[self.front].take();
        // Properly advances the front index with wrapping
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        item
    }
}

/** Illustrates that the for loop is the most efficient way to initialize an
array with None values:

100x
Default: 2.803µs
None: 2.505µs
For: 2.193µs

10000x
Default: 212.307µs
None: 205.76µs
For: 178.989µs

100000000x
Default: 2.285708255s
None: 2.203727308s
For: 1.947812524s

*/
//#[allow(clippy::type_complexity)]
//fn _empirical_test() {
//    use std::time::{Duration, Instant};
//
//    let allocations = [100, 10_000, 100_000_000];
//    let runs = 100;
//    // TODO: Clippy isn't happy about this
//    let methods: Vec<(&str, fn(usize) -> CircularQueue<char>)> = vec![(
//        "For",
//        CircularQueue::new as fn(usize) -> CircularQueue<char>,
//    )];
//
//    for &n in &allocations {
//        println!("\n{}x", n);
//
//        for &(method_name, method) in &methods {
//            let mut avg = Duration::new(0, 0);
//            for _ in 0..runs {
//                let start_time = Instant::now();
//                let _t: CircularQueue<char> = method(n);
//                let duration = start_time.elapsed();
//                avg += duration;
//            }
//            println!("{}: {:?}", method_name, avg / runs);
//        }
//    }
//}
// To avoid Clippy warning suppression:
pub fn empirical_test() {
    use std::time::{Duration, Instant};

    let allocations = [100, 10_000, 100_000_000];
    let runs = 100;

    // Use closures to avoid function pointer type
    let methods = [("For", |n| CircularQueue::<char>::new(n))];

    for &n in &allocations {
        println!("\n{n}x");

        for &(name, constructor) in &methods {
            let total_duration = (0..runs)
                .map(|_| {
                    let start = Instant::now();
                    let _queue = constructor(n);
                    start.elapsed()
                })
                .sum::<Duration>();

            let avg = total_duration / runs;
            println!("{name}: {avg:?}");
        }
    }
}

pub fn example() {
    //TODO
}
