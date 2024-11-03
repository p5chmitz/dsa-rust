//////////////////////////////////
/** A safe, singly-linked queue */
//////////////////////////////////

pub mod linked_queue {
    pub struct Frame<T> {
        data: T,
        next: Option<Box<Frame<T>>>,
    }
    impl<T> Frame<T> {
        // Creates a new frame
        pub fn new(data: T) -> Frame<T> {
            Frame { data, next: None }
        }
    }
    // The Queue API includes
    // - new() -> Queue
    // - enqueue(&mut self, node: Box<Frame>)
    // - peek(&self) -> Option<char>
    // - dequeue(&mut self) -> Option<Frame>
    pub struct Queue<T> {
        front: Option<Box<Frame<T>>>,
        length: usize,
    }
    impl<T> Queue<T> {
        // Creates a new list
        pub fn new() -> Queue<T> {
            Queue {
                front: None,
                length: 0,
            }
        }
        /** Runs in O(n) because I cant (or dont want to try to) safely
         * store a reference to the last node in the list */
        pub fn enqueue(&mut self, frame: Frame<T>) {
            let new_node = Box::new(frame);

            // If the queue is empty, set `self.front` to the new node;
            // Otherwise, traverse to the end of the queue and add the new node
            if self.front.is_none() {
                self.front = Some(new_node);
            } else {
                let mut iter_frame = &mut self.front;
                while let Some(ref mut peek) = iter_frame {
                    if peek.next.is_none() {
                        peek.next = Some(new_node);
                        break;
                    }
                    iter_frame = &mut peek.next;
                }
            }
            self.length += 1;
        }
        /** Returns a reference to the front of the list in O(1) time */
        pub fn peek(&self) -> Option<&T> {
            //if let Some(s) = &(*self).head {
            //    Some(&s.data)
            //} else {
            //    None
            //}
            self.front.as_ref().map(|node| &node.data)
        }
        /** Returns the front of the list (if any) in O(1) time */
        pub fn dequeue(&mut self) -> Option<Frame<T>> {
            if let Some(mut boxed_frame) = self.front.take() {
                self.front = boxed_frame.next.take();
                self.length -= 1;
                Some(*boxed_frame)
            } else {
                None
            }
        }
    }
    #[test]
    fn linked_queue_test() {
        let mut q: Queue<char> = Queue::new();
        // All these shits panic because pushing and popping a full/empty queue is an error
        q.enqueue(Frame::new('a'));
        q.enqueue(Frame::new('b'));
        q.enqueue(Frame::new('c'));

        let a = q.dequeue().unwrap();
        assert_eq!(a.data, 'a');

        q.enqueue(Frame::new('d'));

        // Test that the dequeue is indeed pulling from the front
        // and that the size remains accurate
        q.dequeue().unwrap(); // b
        q.dequeue().unwrap(); // c
        let d = q.dequeue().unwrap();
        assert_eq!(d.data, 'd');
    }
}
