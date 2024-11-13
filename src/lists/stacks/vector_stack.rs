/////////////////////////////////////
/** A vector-based stack (wrapper) */
/////////////////////////////////////

/** Vec itself can be used as a stack with push, pop, and last;
 * This exercise reimplements these operations as add, remove, and peek respectively
 * as well as adding an instance variable size that cannot be negative */
mod wrapper {
    pub struct Stack<T> {
        data: Vec<T>,
        size: usize,
    }
    impl<T> Stack<T> {
        fn new() -> Stack<T> {
            Stack {
                data: Vec::new(),
                size: 0,
            }
        }
        /** Adds an element to the stack */
        fn add(&mut self, e: T) {
            self.size += 1;
            self.data.push(e) // Uses push() from Vec
        }
        /** Peeks at the top of the list without deleting the element */
        fn peek(&self) -> Option<&T> {
            self.data.last()
        }
        /** Returns and deletes the top of the stack */
        fn remove(&mut self) -> Option<T> {
            self.size -= 1;
            self.data.pop()
        }
    }

    /** Example of a nested symbol balancer using a stack;
    Reads each character from the input string;
     - if the char is an opening symbol its pushed to the stack
     - if the char is a closing symbol and the stack size is zero: error
     - if the char is a closing symbol and it matches an opening char on the stac, its popped */
    pub fn balance(s: String) -> bool {
        let mut symbols = Stack::new();

        for e in s.chars() {
            match e {
                '[' | '{' | '(' => {
                    symbols.add(e);
                }
                ']' | '}' | ')' => {
                    // Checks that all closing symbols have a matching opener,
                    // if it does, the opener is popped
                    if symbols.size == 0 {
                        panic!("Error: Unexpected closing symbol");
                    } else {
                        let check = *symbols.peek().expect("Error: No symbols on stack");
                        if (check == '[' && e == ']')
                            || (check == '{' && e == '}')
                            || (check == '(' && e == ')')
                        {
                            symbols.remove();
                        }
                    }
                }
                _ => {}
            }
        }
        // Checks that all opening symbols have a matching closer
        if symbols.size > 0 {
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
        assert!(!balance(input));
    }
    #[test]
    #[should_panic(expected = "Error: Unexpected closing symbol")]
    fn illegal_opening_brace_fail() {
        let input = "}{[]}{}".to_string(); // Fails with leading closing symbol
        assert!(!balance(input));
    }
    #[test]
    #[should_panic(expected = "Error: Missing closing symbol")]
    fn open_block_fail() {
        let input = "{[]}{".to_string();
        assert!(!balance(input));
    }
}

// Illustrates that the previous wrapper was completely unnecessary because the good people
// at Rust already made the Vec type compatible with stack operations
mod raw {
    pub fn balance(s: String) -> bool {
        let mut symbols = Vec::new();

        for e in s.chars() {
            match e {
                '[' | '{' | '(' => {
                    symbols.push(e);
                }
                ']' | '}' | ')' => {
                    // Checks that all closing symbols have a matching opener,
                    // if it does, the opener is popped
                    if symbols.len() == 0 {
                        panic!("Error: Unexpected closing symbol");
                    } else {
                        let check = *symbols.last().expect("Error: No symbols on stack");
                        if (check == '[' && e == ']')
                            || (check == '{' && e == '}')
                            || (check == '(' && e == ')')
                        {
                            symbols.pop();
                        }
                    }
                }
                _ => {}
            }
        }
        // Checks that all opening symbols have a matching closer
        if symbols.len() > 0 {
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
