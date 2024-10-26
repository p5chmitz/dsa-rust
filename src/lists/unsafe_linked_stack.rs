////////////////////////////////////////
/** A horribly unsafe stack (y tho?!) */
////////////////////////////////////////

// A raw pointer to some Frame
type Link = Option<*mut Frame>;

#[derive(Debug, PartialEq)]
pub struct Frame {
    pub symbol: char,
    next: Link,
}
impl Frame {
    pub fn new(symbol: char) -> Frame {
        Frame {
            symbol,
            next: None,
        }
    }
}
// The Stack API includes
// - new() -> Stack
// - push(&mut self, node: Box<Frame>)
// - peek(&self) -> Option<char>
// - pop(&mut self) -> Option<Frame>
// - iter(&self) -> Iter
// - print(&self)
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
    /** Returns the head Frame, if it exists */
    pub fn peek(&self) -> Option<char> {
        if let Some(v) = self.head { 
            unsafe { 
                Some((*v).symbol) 
            } 
        } else { None }
    }
    /** Inserts a frame */
    pub fn push(&mut self, frame: Box<Frame>) {
        // Gets a raw, mutable pointer to the (new, unique) heap-allocated frame
        let new_frame_ptr: *mut Frame = Box::into_raw(frame);

        // Sets the new frame's next pointer to the current head
        unsafe { (*new_frame_ptr).next = self.head; }

        // Resets the stack's head and increments its size
        self.head = Some(new_frame_ptr);
        self.length += 1;
        return
    }
    /** Removes and returns the head frame */
    pub fn pop(&mut self) -> Option<Frame> {
        unsafe {
            if let Some(head) = self.head {
                self.head = (*head).next;
                let popped: Frame = *Box::from_raw(head);
                self.length -= 1;
                Some(popped)
            } else { None }
        }
    }
    // NOTE: There is no need to iterate over the stack
    //pub fn iter(&self) -> Iter {
    //    Iter {
    //        next: self.head.as_ref().map(|&ptr| unsafe { &*ptr }),
    //    }
    //}
    // NOTE: There is no need to print the stack
    //pub fn print(&self) {
    //    let mut counter = 1;
    //    for e in self.iter() {
    //        println!("{:>2}: {:<2}", counter, e.symbol);
    //        counter += 1;
    //    }
    //    println!("")
    //}
}
//pub struct Iter<'a> {
//    next: Option<&'a Frame>,
//}
//impl<'a> Iterator for Iter<'a> {
//    type Item = &'a Frame;
//    /** Returns each Frame in the list until there are None */
//    fn next(&mut self) -> Option<Self::Item> {
//        self.next.take().map(|current| {
//            self.next = current.next.as_ref().map(|&ptr| unsafe { &*ptr });
//            current
//        })
//    }
//}
impl Drop for Stack {
    /** Stack destructor */
    fn drop(&mut self) {
        unsafe {
            let mut current_frame_ptr = self.head;
            while let Some(ptr) = current_frame_ptr {
                // Store a pointer to the next Frame before dropping the current one
                let next_frame_ptr = (*ptr).next;

                // Boxes the current Frame for automatic collection
                let _ = Box::from_raw(ptr);

                // Advance the Frame pointer
                current_frame_ptr = next_frame_ptr;
            }
        }
    }
}

/** Example funciton that uses the stack to check if a String contains balanced sets of braces */
pub fn balance(s: String) -> bool {
    let mut symbols = Stack::new();

    for e in s.chars() {
        let f: Box<Frame> = Box::new(Frame::new(e));
        match e {
            '[' | '{' | '(' => {
                symbols.push(f);
            }
            ']' | '}' | ')' => {
                // Error if there is a closer with an empty list
                if symbols.length == 0 {
                    panic!("Error: Unexpected closing symbol");
                } 
                // Else check for and pop the matching opener
                else {
                    if let Some(check) = symbols.peek() {
                        if (check == '[' && e == ']')
                            || (check == '{' && e == '}')
                            || (check == '(' && e == ')')
                        {
                            symbols.pop();
                        }
                    }
                }
            }
            _ => {}
        }
    }
    // Error if there are any leftover symbols
    if symbols.length > 0 {
        panic!("Error: Missing closing symbol")
    }
    true
}

#[test]
fn basic_mem_test() {
    // Creates a new doubly-linked list
    let mut list = Stack::new();

    // Creates initial head and tail nodes
    let a = Box::new(Frame::new('{'));
    let b = Box::new(Frame::new('}'));
    let c = Box::new(Frame::new('{'));

    unsafe {
        list.push(a); // the list has a head, and its c
        let a_ptr: *mut Frame = list.head.unwrap();
        let a_ref: &mut Frame = &mut *a_ptr;
        assert_eq!(a_ref.symbol, '{');
        assert_eq!(a_ref.next, None);

        // Test case: Replace head (push)
        list.push(b); // head is now b
        let b_ptr: *mut Frame = list.head.unwrap();
        let b_ref: &mut Frame = &mut *b_ptr;
        assert_eq!(b_ref.symbol, '}');
        assert_eq!(b_ref.next, Some(a_ptr));

        list.push(c); // head is now a
        let c_ptr: *mut Frame = list.head.unwrap();
        let c_ref: &mut Frame = &mut *c_ptr;
        assert_eq!(c_ref.symbol, '{');
        assert_eq!(c_ref.next, Some(b_ptr));

    }
}
#[test]
fn success() {
    let input = "{[({[]}[(())]){{}{}{([{{}}])}{{}}{[()()()[{}]]}}]}".to_string();
    assert!(balance(input));

    let input = "{\"name\":\"Peter\",\"age\":40,\"birthplace\":{\"city\":\"Iowa City\",\"state\":\"Iowa\"}}".to_string();
    assert!(balance(input));

}
#[test]
#[should_panic(expected = "Error: Missing closing symbol")]
fn mismatched_symbols_fail() {
    let input = "{{}}{{}".to_string(); // Second set missing closing symbol
    assert!(!balance(input), "");
}
#[test]
#[should_panic(expected = "Error: Missing closing symbol")]
fn missing_closing_fail() {
    let input = "{{}{{}}".to_string(); // First set missing closing symbol
    assert!(!balance(input), "");
}
#[test]
#[should_panic(expected = "Error: Unexpected closing symbol")]
fn illegal_opening_brace_fail() {
    let input = "}{[]}{}".to_string(); // Fails with leading closing symbol
    assert!(!balance(input), "");
}
#[test]
#[should_panic(expected = "Error: Missing closing symbol")]
fn open_block_fail() {
    let input = "{[]}{".to_string(); // Fails with missing trailing closing symbol
    assert!(!balance(input), "");
}

