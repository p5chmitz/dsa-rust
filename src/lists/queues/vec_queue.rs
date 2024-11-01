///////////////////////////
/** A Vector-based queue */
///////////////////////////

pub mod vec_wrapper {
    #[derive(Debug, PartialEq)]
    pub struct Queue {
        data: Vec<char>,
        size: usize,
    }
    impl Queue {
        fn new() -> Queue {
            Queue {
                data: Vec::with_capacity(10),
                size: 0,
            }
        }
        /** Adds an element to the queue */
        fn push(&mut self, s: char) {
            self.size += 1;
            self.data.push(s) // Uses push() from Vec
        }
        /** Peeks at the top of the list without deleting the element */
        fn peek(&self) -> Option<&char> {
            self.data.last()
        }
        /** Returns and deletes the top of the queue */
        fn remove(&mut self) -> Option<char> {
            self.size -= 1;
            Some(self.data.remove(0))
        }
    }
    // Convenience (declarative) macro for building queue! objects like vec!
    // Necessitates explicit allow attribute to suppress warnings because the macro is only used in tests
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
        let first: Queue = queue!('a', 'b', 'c', 'd');
        assert_eq!(first, queue);

        queue.remove();
        queue.remove();
        queue.push('z');
        let second = queue!('c', 'd', 'z');
        assert_eq!(second, queue);
    }
}
