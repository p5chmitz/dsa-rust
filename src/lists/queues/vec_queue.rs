/*! A Vector-based queue (wrapper) 

This is a simple Vec wrapper

*/

pub mod vec_wrapper {
    /** The Queue's API contains the following functions:
    NOTE: All methods run in O(1) time except remove() which runs in O(n). For more efficient operations
    use the VecDeque type. */
    #[derive(Debug, PartialEq)]
    pub struct Queue<T> {
        data: Vec<T>,
        size: usize,
    }
    impl<T> Queue<T> {
        /** Creates a new Queue with an initial capacity of 10 in O(1) time */
        pub fn new() -> Queue<T> {
            Queue {
                data: Vec::with_capacity(10),
                size: 0,
            }
        }
        /** Adds an element to the queue; Amortized to O(1) time */
        pub fn push(&mut self, s: T) {
            self.size += 1;
            self.data.push(s) // Uses push() from Vec
        }
        /** Peeks at the top of the list without deleting the element in O(1) time */
        pub fn peek(&self) -> Option<&T> {
            self.data.last()
        }
        /** Returns and deletes the top of the queue in O(n) time */
        pub fn remove(&mut self) -> Option<T> {
            self.size -= 1;
            Some(self.data.remove(0))
        }
    }
    // Convenience (declarative) macro for building queue! objects like vec!
    // Requires explicit allow attribute to suppress warnings because the macro is only used in tests
    #[allow(unused_macros)]
    macro_rules! queue {
        ($($c:expr),*) => {
            {
                let data = vec![$($c),*];
                let size = data.len();
                Queue {
                    data,
                    size,
                }
            }
        };
    }

    #[test]
    fn vec_queue_test() {
        let mut queue = Queue::new();
        queue.push('a');
        queue.push('b');
        queue.push('c');
        queue.push('d');
        let first: Queue<char> = queue!('a', 'b', 'c', 'd');
        assert_eq!(first, queue);

        queue.remove();
        queue.remove();
        queue.push('z');
        let second: Queue<char> = queue!('c', 'd', 'z');
        assert_eq!(second, queue);
    }
}

/** Illustrates how Vec can be used as a queue by itself; Convenient, but remove(0) runs in O(n) time;
Use VecDeque for flexible operations in O(1) time */
pub mod vec {
    #[test]
    // Illustrates how Vec can be used as a queue, but with some inefficiencies
    fn vec_queue_test() {
        let mut queue = Vec::new();
        queue.push('a');
        queue.push('b');
        queue.push('c');
        queue.push('d');
        let first: Vec<char> = vec!['a', 'b', 'c', 'd'];
        assert_eq!(first, queue);

        // remove() scales in O(n) time
        queue.remove(0);
        queue.remove(0);
        queue.push('z');
        let second: Vec<char> = vec!['c', 'd', 'z'];
        assert_eq!(second, queue);
    }
}
