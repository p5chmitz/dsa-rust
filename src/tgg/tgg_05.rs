#![allow(dead_code)]

// Ch 5: Recursion
//////////////////

// This factorial function is defined with u32 which a) cannot take
// negative values by default and b) has a max return of 
// 4,294,967,295, or 12! because 13! is 6,227,020,800. 
// If you bump it up to u128 you can calculate up to 34!.

// My first attempt
pub fn factorial_0(mut n: u32) -> u32 {
    if n <= 1 {
        return n
    } else {
        let nex = factorial_0(n - 1);
        n *= nex;
    };
    return n
}

// After translating from the book's Java example
// Note that this version uses a signed integer primitive so the function
// also includes a bounds check for values < 0.
pub fn factorial_1(n: i32) -> i32 {
    if n < 0 {
        println!("Error: Cannot compute factorials for n < 0");
        return n
    } else if n == 0 {
        return 1
    } else {
        return n * factorial_1(n - 1)
    }
}

// After analyzing the book's example (cheating)
pub fn factorial_2(n: u32) -> u32 {
    if n <= 1 {
        return n
    } 
    return n * factorial_2(n - 1)
}

// Refactoring the recursive function for an iterative one
pub fn factorial_3(mut n: u32) -> u32 {
    let mut fac = n; 
    while n > 1 { 
        fac *= n - 1; 
        n -= 1; 
    }
    fac
}

// Refactored iterative function for a more logical evaluation (cheat)
pub fn factorial_4(n: u32) -> u32 {
    let mut fac = 1;
    for e in 2..=n {
        fac *= e
    }
    fac
}

pub fn bin_search_0(a: &Vec<i32>, t: i32, left: i32, right: i32) 
    -> i32 {
    // Recursive base case
    if left > right {
        return -1
    } else {
        let mid = (left + right) / 2;
        if t == a[mid as usize] {
            return mid as i32
        } else if t < a[mid as usize] {
            return bin_search_0(&a, t, left, mid - 1)
        } else {
            return bin_search_0(&a, t, mid + 1, right)
        }
    }
}
