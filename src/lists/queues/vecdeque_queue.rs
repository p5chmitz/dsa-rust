/////////////////////////////
/** A VecDeque-based queue */
/////////////////////////////

/** VecDeque is a more efficient implementation than Vec */
mod vecdeque_wrapper {
    use std::collections::VecDeque;

    /** The Queue's API contains the following functions: 
    - new() -> Queue<T>
    - push(&mut self, s: T)
    - peek(&self) -> Option<&T>
    - dequeue(&mut self) -> Option<T>
    NOTE: All methods run in O(1) time */
    pub struct Queue<T> {
        data: VecDeque<T>,
        size: usize,
    }
    impl<T> Queue<T> {
        fn new() -> Queue<T> {
            Queue {
                data: VecDeque::new(),
                size: 0,
            }
        }
        /** Adds an element to the queue */
        fn push(&mut self, e: T) {
            self.size += 1;
            self.data.push_back(e)
        }
        /** Peeks at the top of the list without deleting the element */
        fn peek(&self) -> Option<&T> {
            self.data.front()
        }
        /** Returns and deletes the top of the queue */
        fn dequeue(&mut self) -> Option<T> {
            self.size -= 1;
            self.data.pop_front()
        }
    }
}

/** Illustrates how VecDeque can be used as a queue without a silly wrapper */
pub mod vec_deque {

    use std::collections::VecDeque;

    #[test]
    // First forwards
    fn vecdeque_queue_test_forwards() {
        let mut queue = VecDeque::new();
        queue.push_back('a');
        queue.push_back('b');
        queue.push_back('c');
        queue.push_back('d');
        let first: Vec<char> = vec!('a', 'b', 'c', 'd');
        let first_vec_deque: VecDeque<char> = VecDeque::from(first);
        assert_eq!(first_vec_deque, queue);

        queue.pop_front();
        queue.pop_front();
        queue.push_back('z');
        let second: Vec<char> = vec!('c', 'd', 'z');
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
        let first: Vec<char> = vec!('d', 'c', 'b', 'a');
        let first_vec_deque: VecDeque<char> = VecDeque::from(first);
        assert_eq!(first_vec_deque, queue);

        queue.pop_back();
        queue.pop_back();
        queue.push_front('z');
        let second: Vec<char> = vec!('z', 'd', 'c');
        let second_vec_deque: VecDeque<char> = VecDeque::from(second);
        assert_eq!(second_vec_deque, queue);
    }
}
