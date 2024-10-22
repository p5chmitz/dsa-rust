//////////////////////////////
/** A horribly unsafe stack */
//////////////////////////////

// A raw pointer to some Frame
type Link = Option<*mut Frame>;

#[derive(Debug)]
pub struct Frame {
    pub name: String,
    pub score: i32,
    next: Link,
}
impl Frame {
    // Creates a new node with a unique, heap-allocated address via Box
    pub fn new(name: String, score: i32) -> Box<Frame> {
        Box::new(Frame {
            name,
            score,
            next: None,
        })
    }
}
pub struct Stack {
    head: Link,
    length: usize,
}
impl Stack {
    // Creates a new list
    pub fn new() -> Stack {
        Stack {
            head: None,
            length: 0,
        }
    }
    /** Inserts a node, sorted by its score */
    pub fn push(&mut self, node: Box<Frame>) {
        // Gets a raw, mutable pointer to the (new) unique heap object
        let new_node_ptr: *mut Frame = Box::into_raw(node);

        unsafe {
            // Special case for empty list
            if self.head.is_none() {
                println!("Pushed first node {}", (*new_node_ptr).name);
                // Sets initial head and tail pointers, increments the list size
                self.head = Some(new_node_ptr);
                self.length += 1;
                return;
            }
            // Special case for inserting new head node
            else {
                // Sets the new node's next pointer to the current head
                (*new_node_ptr).next = self.head;

                println!("Pushed new head {}", (*new_node_ptr).name);
                // Resets the list's head and increments the list size
                self.head = Some(new_node_ptr);
                self.length += 1;
                return;
            }
        }
    }
    /** Removes a node at a provided index */
    pub fn pop(&mut self) -> Frame {
        unsafe {
            let head: *mut Frame = self.head.unwrap();
            self.head = (*head).next;
            let popped: Frame = *Box::from_raw(head);
            self.length -= 1;
            popped
        }
    }
    pub fn iter(&self) -> Iter {
        Iter {
            next: self.head.as_ref().map(|&ptr| unsafe { &*ptr }),
        }
    }
    pub fn print(&self) {
        let mut counter = 1;
        for e in self.iter() {
            println!("{:>2}: {:<8} {:>6}", counter, e.name, e.score);
            counter += 1;
        }
        println!("")
    }
}
pub struct Iter<'a> {
    next: Option<&'a Frame>,
}
impl<'a> Iterator for Iter<'a> {
    type Item = &'a Frame;
    /** Returns each Frame in the list until there are None */
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|current| {
            self.next = current.next.as_ref().map(|&ptr| unsafe { &*ptr });
            current
        })
    }
}
impl Drop for Stack {
    /** Stack destructor */
    fn drop(&mut self) {
        unsafe {
            let mut current_node_ptr = self.head;
            while let Some(ptr) = current_node_ptr {
                // Store a pointer to the next Frame before dropping the current one
                let next_node_ptr = (*ptr).next;

                // Boxes the current Frame for automatic collection
                let _ = Box::from_raw(ptr);

                // Advance the Frame pointer
                current_node_ptr = next_node_ptr;
            }
        }
    }
}

#[test]
fn test() {
    // Creates a new doubly-linked list
    let mut list = Stack::new();

    // Creates initial head and tail nodes
    let a = Frame::new("a".to_string(), 101);
    let b = Frame::new("b".to_string(), 91);
    let c = Frame::new("c".to_string(), 73);

    unsafe {
        // Test case: Inserts first Frame (push)
        list.push(c); // the list has a head, and its c
        let c_ptr: *mut Frame = list.head.unwrap();
        let c_ref: &mut Frame = &mut *c_ptr;
        assert_eq!(c_ref.name, "c");
        assert_eq!(c_ref.score, 73);
        assert_eq!(c_ref.next, None);

        // Test case: Replace head (push)
        list.push(b); // head is now b
        let b_ptr: *mut Frame = list.head.unwrap();
        let b_ref: &mut Frame = &mut *b_ptr;
        assert_eq!(b_ref.name, "b");
        assert_eq!(b_ref.score, 91);
        assert_eq!(b_ref.next, Some(c_ptr));

        list.push(a); // head is now a
        let a_ptr: *mut Frame = list.head.unwrap();
        let a_ref: &mut Frame = &mut *a_ptr;
        assert_eq!(a_ref.name, "a");
        assert_eq!(a_ref.score, 101);
        assert_eq!(a_ref.next, Some(b_ptr));

        // Test case: Remove head (pop)
        let p = list.pop(); // head should be a
        assert_eq!(p.name, "a");
        assert_eq!(p.score, 101);
        assert_eq!(p.next, Some(b_ptr));
    }
}

/** Runs example operations to demonstrate functionality */
pub fn example() {
    let mut list = Stack::new();

    let mut node = Frame::new("Peter".to_string(), 1223);
    list.push(node);

    node = Frame::new("Brain".to_string(), 616);
    list.push(node);

    node = Frame::new("Remus".to_string(), 1225);
    list.push(node);

    node = Frame::new("Bobson".to_string(), 69);
    list.push(node);

    node = Frame::new("Dorkus".to_string(), 412);
    list.push(node);

    node = Frame::new("Dongus".to_string(), 873);
    list.push(node);

    let mut popped: Frame = list.pop();
    println!("Popped {}", popped.name);
    popped = list.pop();
    println!("Popped {}", popped.name);
    popped = list.pop();
    println!("Popped {}", popped.name);

    // Print this bih
    println!("The final list contains {} results:", list.length);
    list.print();
}
