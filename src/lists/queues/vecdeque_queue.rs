/////////////////////////////
/** A VecDeque-based queue */
/////////////////////////////

mod vecdeque_wrapper {
    // VecDeque is a more efficient implementation than Vec
    use std::collections::VecDeque;

    pub struct Queue {
        data: VecDeque<char>,
        size: usize,
    }
    impl Queue {
        fn new() -> Queue {
            Queue {
                data: VecDeque::new(),
                size: 0,
            }
        }
        /** Adds an element to the queue */
        fn push(&mut self, s: char) {
            self.size += 1;
            self.data.push_back(s) // Uses push() from Vec
        }
        /** Peeks at the top of the list without deleting the element */
        fn peek(&self) -> Option<&char> {
            self.data.front()
        }
        /** Returns and deletes the top of the queue */
        fn dequeue(&mut self) -> Option<char> {
            self.size -= 1;
            self.data.pop_front()
        }
    }
}
