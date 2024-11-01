/////////////////////////////////
/** A circular Vec-based queue */
/////////////////////////////////

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
        //data.resize_with(capacity, Default::default); // Clean and generic
        //data.resize_with(capacity, || None); // More explicit and slightly more efficient
        for _ in 0..capacity { // Somehow the fastest 
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

/** Illustrates that the for loop is the most efficient way to initialize an array with None values 
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
 * */
pub fn empirical_test() {
    use std::time::{Duration, Instant};

    let allocations = [100, 10_000, 100_000_000];
    let runs = 100;
    let methods: Vec<(&str, fn(usize) -> CircularQueue<char>)> = vec![
        // Requires separate new2/3 implementations
        //("Default", CircularQueue::new2 as fn(usize) -> CircularQueue<char>),
        //("None", CircularQueue::new3 as fn(usize) -> CircularQueue<char>),
        ("For", CircularQueue::new as fn(usize) -> CircularQueue<char>),
    ];

    for &n in &allocations {
        println!("\n{}x", n);
        
        for &(method_name, method) in &methods {
            let mut avg = Duration::new(0, 0);
            for _ in 0..runs {
                let start_time = Instant::now();
                let _t: CircularQueue<char> = method(n);
                let duration = start_time.elapsed();
                avg += duration;
            }
            println!("{}: {:?}", method_name, avg / runs);
        }
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

/** Illustrates a Josephus Problem solution */
pub fn circular_queue_example() {

}

