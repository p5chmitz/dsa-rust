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
    /** The CircularQueue's public API contains the following functions: 
     * - new(capacity: usize) -> CircularQueue<T>
     * - enqueue(&mut self, item: T) -> Result<(), &str>
     * - dequeue(&mut self) -> Option<T>
     * NOTE: All functions operation in O(1) time */
    impl<T> CircularQueue<T> {
        /** Creates a queue that contains `capacity` number of elements in O(1) time */
        pub fn new(capacity: usize) -> CircularQueue<T> {
            let mut data = Vec::with_capacity(capacity);
            // Fills the vec with None
            data.resize_with(capacity, Default::default); // The most efficient way
            //data.resize_with(capacity, || None); // More explicit, but less efficient
            //for _ in 0..capacity { // 4x slower than resize_with using Default
            //    data.push(None)
            //}
            CircularQueue {
                data,
                front: 0,
                back: 0,
                size: 0,
                capacity,
            }
        }
        /** Adds an element to the back of the queue in O(1) time */
        pub fn enqueue(&mut self, item: T) -> Result<(), &str> {
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
        /** Removes and returns the front element of the queue in O(1) time */
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
        // All these shits panic because pushing and popping a full/empty queue is an error
        q.enqueue('a').unwrap(); 
        q.enqueue('b').unwrap(); 
        q.enqueue('c').unwrap(); 

        assert_eq!(0, q.capacity - q.size); // The number of empty indexes
        assert_eq!(q.size, q.capacity); // That the queue is now at capacity
        assert_eq!(q.front, 0); // The front is still zero                                
        assert_eq!(q.back, 2); // The last operation wrote to the last available position

        assert!(q.enqueue('d').is_err()); // That the queue cant take additional elements

        let a = q.dequeue().unwrap();
        assert_eq!(a, 'a'); // Tests that the dequeue pulls from the front of the queue
        assert_eq!(q.size, 2); // Test that the size is being appropriately decremented too
        assert_eq!(1, q.capacity - q.size); // Tests that dequeue is "freeing" indexes
        assert_eq!(q.front, 1); // The front advanced by one with dequeue                               
        assert_eq!(q.back, 2); // The back is still 2

        // Test that the dequeue is indeed pulling from the front
        // and that the size remains accurate
        q.dequeue().unwrap(); 
        q.dequeue().unwrap(); 
        q.enqueue('d').unwrap(); // That the queue can take additional elements again
        let d = q.dequeue().unwrap();
        assert_eq!(d, 'd'); // And that they're correct

        // Final state of the queue
        assert_eq!(q.front, 1);
        assert_eq!(q.back, 0);
        assert_eq!(q.size, 0);
    }
}
/** Illustrates a Josephus Problem solution */
pub fn circular_queue_example() {

}

