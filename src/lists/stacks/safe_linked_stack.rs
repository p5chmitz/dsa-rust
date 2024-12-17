//////////////////////////////////
/** A safe, singly-linked stack */
//////////////////////////////////

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    // Creates a new frame
    pub fn new(data: T) -> Node<T> {
        Node { data, next: None }
    }
}
/** The Stack API includes
 - new() -> Stack<T>
 - push(&mut self, frame: Box<Node<T>>)
 - peek(&self) -> Option<&T>
 - pop(&mut self) -> Option<Node<T>>
*/
pub struct Stack<T> {
    head: Option<Box<Node<T>>>, // Adding an extra box just in case things get wild
    length: usize,
}
impl<T> Stack<T> {
    // Creates a new list
    pub fn new() -> Stack<T> {
        Stack {
            head: None,
            length: 0,
        }
    }
    pub fn push(&mut self, frame: Box<Node<T>>) {
        let mut new_head = frame;
        new_head.next = self.head.take();
        self.head = Some(new_head);
        self.length += 1;
        return;
    }
    pub fn peek(&self) -> Option<&T> {
        if let Some(s) = &(*self).head {
            Some(&s.data)
        } else {
            None
        }
    }
    pub fn pop(&mut self) -> Option<Node<T>> {
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
    use super::{Node, Stack};

    /** Example funciton that uses the stack to check if a String contains balanced sets of braces */
    pub fn balance(string: String) -> bool {
        let mut symbols = Stack::new();

        for element in string.chars() {
            // Heap-allocates a Node containing an individual char from s
            let node: Box<Node<char>> = Box::new(Node::new(element));
            match element {
                '[' | '{' | '(' => {
                    symbols.push(node);
                }
                ']' | '}' | ')' => {
                    // Error if there is a closer with an empty list
                    if symbols.length == 0 {
                        panic!("Error: Unexpected closing symbol");
                    }
                    // Else check for and pop the matching opener
                    else {
                        if let Some(check) = symbols.peek() {
                            // matches the next element with the top of the stack
                            // and pops it if theres a match
                            if (*check == '[' && element == ']')
                                || (*check == '{' && element == '}')
                                || (*check == '(' && element == ')')
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
        // 🧘
        let input = "{[({[]}[(())]){{{}}{[()()()[{}]]}}]}".to_string();
        assert!(balance(input));
    }
    #[test]
    #[should_panic(expected = "Error: Unexpected closing symbol")]
    fn should_fail() {
        // Fails somewhere mid-string
        let input = "{{{}}{{}}}{{}{}{{{}{}}}}}{{{}{}".to_string();
        assert!(!balance(input), "");

        // Fails with leading closing symbol
        let input = "}{[]}{}".to_string();
        assert!(!balance(input), "");

        // Fails with missing closing symbol
        let input = "{[]}{".to_string();
        assert!(!balance(input), "");
    }
}
