/*! A sorted, Vec-based priority queue

# About
Naive implementation of a priority queue that requires _O(n)_ operations for most things.

*/

// A sorted, array-based priority queue
mod sorted_list {
    pub use crate::lists::queues::traits::PriorityQueue; // Re-exports the trait

    pub struct Entry<K, V> {
        key: K,
        value: V,
    }
    impl<K, V> Entry<K, V> {
        // Creates a new Entry from a key/value pair
        fn new(key: K, value: V) -> Entry<K, V> {
            Entry { key, value }
        }
    }
    /** Defines a MIN queue where the key's value is inversely proportional to its
    priority; Based on a sorted Vec of Entry<K, V> where K is the key,
    and V is the value ("payload") */
    pub struct SortedVecQueue<K, V> {
        pub data: Vec<Entry<K, V>>,
    }
    impl<K, V> SortedVecQueue<K, V> {
        // Creates a new, empty priority queue structure
        pub fn new() -> SortedVecQueue<K, V> {
            SortedVecQueue {
                data: Vec::with_capacity(0),
            }
        }
    }
    impl<K, V> PriorityQueue<K, V> for SortedVecQueue<K, V>
    where
        K: Ord,
    {
        type Entry = Entry<K, V>;

        //NOTE: Provides a wrapper for Vec::insert() which runs in O(n) time
        fn enqueue(&mut self, key: K, value: V) -> Result<(), Box<dyn std::error::Error>> {
            if Self::check_key(&key) {
                let mut insertion_index = self.data.len();
                // Finds the correct insertion index
                for (i, e) in self.data.iter().enumerate() {
                    if key >= e.key {
                        insertion_index = i;
                        break;
                    }
                }
                let entry = Entry::new(key, value);
                self.data.insert(insertion_index, entry);
                Ok(())
            } else {
                Err("Invalid key".into())
            }
        }

        //NOTE: Provides a wrapper for core::slice::last() which I THINK runs in O(1)...
        fn peek(&self) -> Option<&V> {
            if let Some(v) = self.data.last() {
                Some(&v.value)
            } else {
                None
            }
        }

        //NOTE: Provides a wrapper for Vec::pop() which runs in O(1) time
        fn dequeue(&mut self) -> Option<V> {
            if let Some(v) = self.data.pop() {
                Some(v.value)
            } else {
                None
            }
        }

        fn size(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn compare(one: Self::Entry, two: Self::Entry) -> isize {
            //if one.key < two.key {
            //    -1
            //} else if one.key == two.key {
            //    0
            //} else {
            //    1
            //}
            use std::cmp::Ordering;
            match one.key.cmp(&two.key) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            }
        }

        fn check_key(_key: &K) -> bool {
            //key == key
            true
        }
    }
}

#[test]
pub fn example() {
    use crate::lists::queues::priority_queue::sorted_list::{PriorityQueue, SortedVecQueue};

    // Instantiates new list, declares the K and V types
    //let mut list: SortedVecQueue<usize, &str> = SortedVecQueue::new();
    let mut list: SortedVecQueue<usize, &str> = SortedVecQueue::new();

    // Pushes a bunch of values with an associated key priority to the list
    list.enqueue(3, "Peter").ok();
    list.enqueue(5, "Bobson").ok();
    list.enqueue(2, "Brain").ok();
    list.enqueue(4, "Dingus").ok();
    list.enqueue(6, "Dorkus").ok();

    // Checks that the list is taking entries properly,
    // and that the peek() operation matches expectations
    assert_eq!(list.size(), 5);
    assert_eq!(list.peek(), Some("Brain").as_ref());

    // Creates a "sorted" list of dequeued items
    let mut queue: Vec<&str> = Vec::new();
    // Using the while loop avoids having to deal with partial moves in a for loop
    while !list.is_empty() {
        if let Some(v) = list.dequeue() {
            queue.push(v)
        }
    }

    // Checks that the final result of the queue logic is correct
    assert_eq!(queue, vec!["Brain", "Peter", "Dingus", "Bobson", "Dorkus"])
}

#[test]
fn idk() {
    // Owned and borrowed data
    let x: String = String::from("Peter");
    let y: &String = &x;

    println!("{}", x);
    println!("{}", y);
    println!("{}", *y);

    // Multiple borrows
    let x: String = String::from("Peter");
    let _y: &String = &x;
    let _z: &String = &x;
    println!("{x}");

    // Strong typing and type annotations
    let str_num = String::from("1223");
    //let num: i32 = str_num; // Illegal mismatched type error
    let _num: i32 = str_num.parse().expect("Error"); // Legal cast

    // Passing ownership
    let x: String = String::from("Peter");
    let y: &String = &x;

    println!("{}", x);
    println!("{}", y);
    println!("{}", *y);

    // Mutable data
    let mut refer = String::from("Peter");
    refer += ", the author";
    assert_eq!(refer, "Peter, the author".to_string());

    let mut mutable = String::from("Peter");
    mutable.push_str(", the author"); // Directly modifies the original `String`
    mutable += " ";

    // Mutable borrows
    let mut refer = String::from("Peter");
    let x = &mut refer; // Mutable borrow
    *x = String::from("The author");
    assert_eq!(refer, "The author".to_string());
}
fn passing_ownership() {
    let s = String::from("Peter");
    let ref_s = &s;

    println!("{}", s);
    println!("{}", *ref_s);

    // Both s and ref_s go out of scope here
    let passed = take_and_give_back(s);

    // Illegal use of moved values
    //println!("{}", s);
    //println!("{}", *ref_s);

    println!("{}", passed);
}

fn take_and_give_back(x: String) -> String {
    x
}

#[test]
fn big_things() {
    use std::alloc::Layout;
    use std::mem;

    let name = String::from("Peter");

    // Size of the String object (stack part)
    let stack_size = mem::size_of_val(&name);

    // Size of the heap-allocated string data
    let heap_size = name.len(); // The length of the string, which is the size of the heap-allocated data

    // Print both sizes
    eprintln!("Size of the stack portion (String struct): {}", stack_size);
    eprintln!("Size of the heap portion (string content): {}", heap_size);

    // We cannot directly get memory details about ownership, but the total value size (stack + heap) is:
    let total_size = stack_size + heap_size;
    eprintln!(
        "Total size of the value associated with name: {}",
        total_size
    );

    // Optionally, checking the layout of the String struct
    let layout = Layout::for_value(&name);
    eprintln!("Layout size: {:?}", layout);

    // Testing borrows and the dereference operator
    let x: String = String::from("Peter");
    let y: &String = &x;

    let test = String::from("Peter");
    assert_eq!(x, test);
    assert_eq!(*y, test);

    // Passing ownership
    let s = String::from("Peter");

    let passed = take_and_give_back(s); // s goes out of scope here

    // Illegal use of moved value
    //println!("{}", s);

    let test = String::from("Peter");
    assert_eq!(passed, test);
}
