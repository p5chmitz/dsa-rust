/*! A safe, singly-linked stack 

# About
This simple, `Box`-based stack contains a rudimentary set of operations.

```rust
use dsa_rust::lists::stacks::safe_linked_stack::Stack;

// Creates a stack
let mut stack = Stack::new();

stack.push("Peter");
stack.push("Paul");
stack.push("Mary");

assert_eq!(stack.pop().unwrap(), "Mary");
assert_eq!(stack.pop().unwrap(), "Paul");

stack.push("John");

assert_eq!(stack.pop().unwrap(), "John");

```
*/

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
/// The Stack API
pub struct Stack<T> {
    head: Option<Box<Node<T>>>, // Adding an extra box just in case things get wild
    length: usize,
}
impl<T> Stack<T> {

    /// Creates a new stack
    pub fn new() -> Stack<T> {
        Stack {
            head: None,
            length: 0,
        }
    }

    /// Pushes a type generic over T to the stack
    pub fn push(&mut self, input: T) {
        let ptr = Box::new(Node::new(input));
        let mut new_head = ptr;
        new_head.next = self.head.take();
        self.head = Some(new_head);
        self.length += 1;
        return;
    }

    /// Returns a reference to the data at the head of the stack, if Some
    pub fn peek(&self) -> Option<&T> {
        if let Some(s) = &(*self).head {
            Some(&s.data)
        } else {
            None
        }
    }

    /// Returns the owned data at the head of the stack, if Some
    pub fn pop(&mut self) -> Option<T> {
        if let Some(mut boxed_frame) = self.head.take() {
            self.head = boxed_frame.next.take();
            self.length -= 1;
            Some(boxed_frame.data)
        } else {
            None
        }
    }
}

pub mod safe_stack {
    //use super::{Node, Stack};
    use super::Stack;

    /** Example funciton that uses the stack to check if a String contains balanced sets of braces */
    pub fn balance(string: String) -> bool {
        let mut symbols = Stack::new();

        for element in string.chars() {
            // Heap-allocates a Node containing an individual char from s
            //let node: Box<Node<char>> = Box::new(Node::new(element));
            match element {
                '[' | '{' | '(' => {
                    //symbols.push(node);
                    symbols.push(element);
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
