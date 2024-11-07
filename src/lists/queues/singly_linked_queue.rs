//////////////////////////////////
/** A safe, singly-linked queue */
//////////////////////////////////

/** An inefficient but safe singly-linked queue implementation where everything runs in linear time */
pub mod linked_queue {
    pub struct Node<T> {
        data: T,
        next: Option<Box<Node<T>>>,
    }
    impl<T> Node<T> {
        // Creates a new node
        pub fn new(data: T) -> Node<T> {
            Node { data, next: None }
        }
    }
    /** The Queue API includes the following functions:
    - new() -> Queue
    - enqueue(&mut self, node: Box<Node>)
    - peek(&self) -> Option<char>
    - dequeue(&mut self) -> Option<Node>
    NOTE: This is kind of a terrible implementation as enqueue() runs in O(n) time
    */
    pub struct Queue<T> {
        front: Option<Box<Node<T>>>,
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
        pub fn enqueue(&mut self, node: Node<T>) {
            let new_node = Box::new(node);

            // If the queue is empty, set `self.front` to the new node;
            // Otherwise, traverse to the end of the queue and add the new node
            if self.front.is_none() {
                self.front = Some(new_node);
            } else {
                let mut iter_node = &mut self.front;
                while let Some(ref mut peek) = iter_node {
                    if peek.next.is_none() {
                        peek.next = Some(new_node);
                        break;
                    }
                    iter_node = &mut peek.next;
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
        pub fn dequeue(&mut self) -> Option<Node<T>> {
            if let Some(mut boxed_node) = self.front.take() {
                self.front = boxed_node.next.take();
                self.length -= 1;
                Some(*boxed_node)
            } else {
                None
            }
        }
    }
    #[test]
    fn linked_queue_test() {
        let mut q: Queue<char> = Queue::new();

        q.enqueue(Node::new('a'));
        q.enqueue(Node::new('b'));
        q.enqueue(Node::new('c')); // list: a, b, c

        // Tests that the front is really the front
        let a = q.dequeue().unwrap();
        assert_eq!(a.data, 'a'); // list: b, c

        q.enqueue(Node::new('d')); // list: b, c, d

        q.dequeue().unwrap(); // list: c, d
        q.dequeue().unwrap(); // list: d
        let d = q.dequeue().unwrap();
        assert_eq!(d.data, 'd'); // Now its empty, are you happy?
    }
}
