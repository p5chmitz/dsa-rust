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

// Translated from the book's Java example in O(n) time
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
/** Recursive implementation of a factorial calculator up to 12! in O(n) time */
pub fn factorial_2(n: u32) -> u32 {
    if n <= 1 {
        return n
    } 
    return n * factorial_2(n - 1)
}
// Refactoring the recursive function for an iterative one
/** Iterative implementation of a factorial calculator up to 12! in O(n) time */
pub fn factorial_3(mut n: u32) -> u32 {
    let mut fac = n; 
    while n > 1 { 
        fac *= n - 1; 
        n -= 1; 
    }
    fac
}
// Refactored iterative function for a more logical evaluation (cheat)
/** Iterative implementation of a factorial calculator up to 12! in O(n) time */
pub fn factorial_4(n: u32) -> u32 {
    let mut fac = 1;
    for e in 2..=n {
        fac *= e
    }
    fac
}

/** Recursive implementation of a binary search in O(log n) time */
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

// Initially it appears this algorithm runs in O(n^2) time, but it actually 
// runs in O(n) time because it touches (and performs O(1) operations) on
// n nodes in the tree exactly once.
/** Walks a directory tree printing out names and sizes in O(n) time */
use std::path::Path;
pub fn disk_usage(root: &Path) -> u64 {
    let mut dir_size = 0;
    if root.is_dir() {
        for e in root.read_dir().expect("read_dir call failed") {
            let entry = e.expect("failure to deconstruct value");
            dir_size += disk_usage(&entry.path());
            //if let Ok(entry) = e {
            //    dir_size += disk_usage(&entry.path());
            //}
        }
        let this_dir = std::fs::metadata(root).expect("metadata call failed [0]").len();
        //dir_size += this_dir;
        println!("d {:>7}B  {}", dir_size + this_dir, root.display());
    } else if root.is_file() { 
        let size = std::fs::metadata(root).expect("metadata call failed [1]").len();
        println!("  {:>7}B  {}", size, root.display());
        return size;
    }
    return dir_size;
}

// Sum of array of integers to n indexes 
// Iterative implementation (so easy, so intuitive)
pub fn array_sum_0(v: Vec<i32>) -> i32 {
    let mut sum = 0;
    for e in 0..v.len() {
        sum += v[e]
    }
    sum
}
// Reimplementation using an iterator instead of a range
pub fn array_sum_1(v: Vec<i32>) -> i32 {
    let mut sum = 0;
    for e in v.iter() {
        sum += v[*e as usize]
    }
    sum
}
// Recursive implementation (is dumb)
pub fn array_sum_2(data: &Vec<i32>, n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else {
        return array_sum_2(&data, n - 1) + &data[n as usize - 1];
    }
}
#[test]
pub fn array_sum_test() {
    // Iterative impelmentation test
    let t = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(21, array_sum_0(t));
    // Recursive implementation test
    // Dumb, error prone implementation
    let t = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(21, array_sum_2(&t, 6));
}

// Iteratively reverses the elements of an array 
/** Iteratively reverses the elements of an array in O(n) time */
pub fn array_reversal_0(mut v: Vec<i32>) -> Vec<i32> {
    let mut high = v.len() - 1; // Match the index values, not # of elements
    let mut low  = 0;
    let mut temp;
    println!("Iterative approach\nOriginal: {:?}", v);
    while high > low {
        temp = v[low];
        v[low] = v[high];
        v[high] = temp;
        high -= 1;
        low += 1;
    }
    println!("Reversed: {:?}", v);
    return v;
}
// Recursively reverses the elements of an array in place with liberal type conversion
pub fn array_reversal_1(v: &mut Vec<i32>, low: i32, high: i32) -> &mut Vec<i32> {
    if low < high {
        let temp = v[low as usize];
        v[low as usize] = v[high as usize];
        v[high as usize] = temp; 
        array_reversal_1(v, low + 1, high - 1);
    }
    v
}
// Recursive reimplementation but everything is usize
pub fn array_reversal_2(v: &mut Vec<usize>, low: usize, high: usize) -> &mut Vec<usize> {
    if low < high {
        let temp = v[low];
        v[low] = v[high];
        v[high] = temp; 
        array_reversal_2(v, low + 1, high - 1);
    }
    v
}
#[test]
pub fn array_reversal_test() {
    // Tests the iterative approach
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let rev = vec![8, 7, 6, 5, 4, 3, 2, 1];
    assert_eq!(array_reversal_0(v), rev);

    // Tests the dumb recursive approach
    let mut v = vec![11, 22, 33, 44, 55, 66, 77, 88];
    let rev = vec![88, 77, 66, 55, 44, 33, 22, 11];
    let high = v.len() as i32 - 1;
    array_reversal_1(&mut v, 0, high);
    assert_eq!(v, rev) 
}

// EXTRA CREDIT
///////////////

// The internet's version of the recursive solution
/** A recursive implementation of the Tower of Hanoi solution that
 * runs in O(2^n) time. This algorithm works by breaking the
 * problem set into source, auxiliary, and destination pegs. */
pub fn tower_of_hanoi(n: u32, src: char, dest: char, aux: char) {
    if n == 1 {
        println!("Move disk 1 from peg {} to peg {}", src, dest);
        return;
    }
    tower_of_hanoi(n - 1, src, aux, dest);
    println!("Move disk {} from peg {} to peg {}", n, src, dest); // Trace
    tower_of_hanoi(n - 1, aux, dest, src);
}

