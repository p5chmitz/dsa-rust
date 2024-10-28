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

/////////////////////////////////
/** A circular Vec-based queue */
/////////////////////////////////

pub mod circular_queue {
    //#[derive(Default)] // Required for generic array initialization
    pub struct CircularQueue<T> {
        pub data: Vec<Option<T>>, // Store elements as `Option` to allow reusing slots
        front: usize,
        back: usize,
        size: usize,
        capacity: usize,
    }

    impl<T> CircularQueue<T> {
        /** Creates a queue that contains `capacity` number of elements */
        pub fn new(capacity: usize) -> CircularQueue<T> {
            // Use `with_capacity` and then populate with `None`
            let mut data = Vec::with_capacity(capacity);
            data.resize_with(capacity, || None); // Fills `data` with `None`

            CircularQueue {
                data,
                front: 0,
                back: 0,
                size: 0,
                capacity,
            }
        }
        /** Adds an element to the queue */
        pub fn enqueue(&mut self, item: T) -> Result<(), &str> {
            // Ensures that the queue cannot take more elements than its designated capacity
            if self.size == self.capacity {
                return Err("Queue is full");
            }
            // Writes the value to the proper position
            self.data[self.back] = Some(item);
            // Properly advances the back index with wrapping
            self.back = (self.back + 1) % self.capacity;
            self.size += 1;
            Ok(())
        }
        /** Removes and returns the front element of the queue */
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

    #[test]
    fn circular_queue_test() {
        let mut q: CircularQueue<char> = CircularQueue::new(3);
        let mut remaining = 0;
        // All these shits panic because pushing and popping a full/empty queue is an error
        q.enqueue('a').unwrap();
        q.enqueue('b').unwrap();
        q.enqueue('c').unwrap();

        assert_eq!(remaining, q.capacity - q.size);
        assert_eq!(q.size, q.capacity); // Test that the queue is now at capacity
        assert_eq!(q.back, 0); // Test that the next available position has wrapped

        let a = q.dequeue().unwrap();
        assert_eq!(a, 'a');
        assert_eq!(q.size, 2); // Test that the size is being appropriately decremented too

        // Push the queue back to capacity and ensure it cant take additional elements
        q.enqueue('d').unwrap();
        assert!(q.enqueue('e').is_err());

        // Test that the dequeue is indeed pulling from the front
        // and that the size remains accurate
        q.dequeue().unwrap(); // b
        remaining = 1;
        assert_eq!(remaining, q.capacity - q.size);
        q.dequeue().unwrap(); // c
        let d = q.dequeue().unwrap();
        assert_eq!(d, 'd');

        // Test that the back has wrapped all the way around past the start
        // and that there are no more elements in the queue; your work is done!
        assert_eq!(q.back, 1);
        assert_eq!(q.size, 0);
    }
}

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
        pub fn peek(&self) -> Option<&T> {
            //if let Some(s) = &(*self).head {
            //    Some(&s.data)
            //} else {
            //    None
            //}
            self.front.as_ref().map(|node| &node.data)
        }
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
