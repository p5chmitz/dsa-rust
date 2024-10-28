//////////////////////////////////
/** A safe, singly-linked stack */
//////////////////////////////////

pub struct Frame {
    symbol: char,
    next: Option<Box<Frame>>,
}
impl Frame {
    // Creates a new frame
    pub fn new(symbol: char) -> Frame {
        Frame { symbol, next: None }
    }
}
// The Stack API includes
// - new() -> Stack
// - push(&mut self, node: Box<Frame>)
// - peek(&self) -> Option<char>
// - pop(&mut self) -> Option<Frame>
pub struct Stack {
    head: Option<Box<Frame>>, // Adding an extra box just in case things get wild
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
    pub fn push(&mut self, frame: Box<Frame>) {
        let mut new_head = frame;
        println!("Pushed {} to the stack", &new_head.symbol);
        new_head.next = self.head.take();
        self.head = Some(new_head);
        self.length += 1;
        return;
    }
    pub fn peek(&self) -> Option<char> {
        if let Some(s) = &(*self).head {
            Some(s.symbol)
        } else {
            None
        }
    }
    pub fn pop(&mut self) -> Option<Frame> {
        if let Some(mut boxed_frame) = self.head.take() {
            self.head = boxed_frame.next.take();
            self.length -= 1;
            Some(*boxed_frame)
        } else {
            None
        }
    }
}

pub mod safe_stack {
    use super::{Frame, Stack};

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
    fn success() {
        let input = "{[({[]}[(())]){{{}}{[()()()[{}]]}}]}".to_string();
        assert!(balance(input));
    }
    #[test]
    #[should_panic(expected = "Error: Unexpected closing symbol")]
    fn mismatched_symbols_fail() {
        let input = "{{{}}{{}}}{{}{}{{{}{}}}}}{{{}{}".to_string(); // Fails somewhere mid-string
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
        let input = "{[]}{".to_string();
        assert!(!balance(input), "");
    }
}
