/*! A VecDeque-based queue (wrapper) 

This is a simple std::VecDeque wrapper

```rust
use crate::lists::queue::vecdeque_wrapper;

let mut queue = Queue::new();
queue.enqueue('a');
queue.enqueue('b');
queue.enqueue('c');
queue.enqueue('d');
let first: Queue<char> = queue!('a', 'b', 'c', 'd');
assert_eq!(first, queue);

queue.dequeue();
queue.dequeue();
queue.enqueue('z');
let second: Queue<char> = queue!('c', 'd', 'z');
assert_eq!(second, queue);
```

*/

/** VecDeque is a more efficient implementation than Vec */
mod vecdeque_wrapper {
    use std::collections::VecDeque;

    /** The Queue's API contains the following functions:
    NOTE: All methods run in O(1) time */
    #[derive(Debug, PartialEq)] // Used for declarative macro & testing operations
    pub struct Queue<T> {
        data: VecDeque<T>,
        size: usize,
    }
    #[allow(unused)] // Not actually used anywhere, just an illustration :(
    impl<T> Queue<T> {
        /// Creates a new Queue wrapper with VecDeque backing structure and independent size
        /// variable
        pub fn new() -> Queue<T> {
            Queue {
                data: VecDeque::new(),
                size: 0,
            }
        }
        /** Adds an element to the queue */
        pub fn enqueue(&mut self, e: T) {
            self.size += 1;
            self.data.push_back(e)
        }
        /** Peeks at the top of the list without deleting the element */
        pub fn peek(&self) -> Option<&T> {
            self.data.front()
        }
        /** Returns and deletes the top of the queue */
        pub fn dequeue(&mut self) -> Option<T> {
            self.size -= 1;
            self.data.pop_front()
        }
    }
    // Convenience (declarative) macro for building queue! objects like vec!
    // Requires explicit allow attribute to suppress warnings because the macro is only used in tests
    #[allow(unused_macros)]
    macro_rules! queue {
        ($($c:expr),*) => {
            {
                let mut data = VecDeque::new();
                $(data.push_back($c);)*
                let size = data.len();
                Queue {
                    data,
                    size,
                }
            }
        };
    }

    #[test]
    fn wrapper_test() {
        let mut queue = Queue::new();
        queue.enqueue('a');
        queue.enqueue('b');
        queue.enqueue('c');
        queue.enqueue('d');
        let first: Queue<char> = queue!('a', 'b', 'c', 'd');
        assert_eq!(first, queue);

        queue.dequeue();
        queue.dequeue();
        queue.enqueue('z');
        let second: Queue<char> = queue!('c', 'd', 'z');
        assert_eq!(second, queue);
    }
}

/** Illustrates how VecDeque can be used as a queue without a silly wrapper */
pub mod vec_deque {

    #[allow(unused_imports)] // Not actually used anywhere, just an illustration :(
    use std::collections::VecDeque;

    #[test]
    // First forwards
    fn vecdeque_queue_test_forwards() {
        let mut queue = VecDeque::new();
        queue.push_back('a');
        queue.push_back('b');
        queue.push_back('c');
        queue.push_back('d');
        let first: Vec<char> = vec!['a', 'b', 'c', 'd'];
        let first_vec_deque: VecDeque<char> = VecDeque::from(first);
        assert_eq!(first_vec_deque, queue);

        queue.pop_front();
        queue.pop_front();
        queue.push_back('z');
        let second: Vec<char> = vec!['c', 'd', 'z'];
        let second_vec_deque: VecDeque<char> = VecDeque::from(second);
        assert_eq!(second_vec_deque, queue);
    }

    #[test]
    // And then backwards
    fn vecdeque_queue_test_backwards() {
        let mut queue = VecDeque::new();
        queue.push_front('a');
        queue.push_front('b');
        queue.push_front('c');
        queue.push_front('d');
        let first: Vec<char> = vec!['d', 'c', 'b', 'a'];
        let first_vec_deque: VecDeque<char> = VecDeque::from(first);
        assert_eq!(first_vec_deque, queue);

        queue.pop_back();
        queue.pop_back();
        queue.push_front('z');
        let second: Vec<char> = vec!['z', 'd', 'c'];
        let second_vec_deque: VecDeque<char> = VecDeque::from(second);
        assert_eq!(second_vec_deque, queue);
    }
}
