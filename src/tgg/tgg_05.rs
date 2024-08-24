#![allow(dead_code)]

// Ch 5: Recursion
//////////////////

// This factorial function is defined with u32 which a) cannot take
// negative values by default and b) has a max return of
// 4,294,967,295, or 12! because 13! is 6,227,020,800.
// If you bump it up to u128 you can calculate up to 34!.
/** Computs a n! up to 12 in O(n) time */
pub fn factorial_0(mut n: u32) -> u32 {
    if n <= 1 {
        return n;
    } else {
        let nex = factorial_0(n - 1);
        n *= nex;
    };
    return n;
}
// Translated from the book's Java example in O(n) time
// Note that this version uses a signed integer primitive so the function
// also includes a bounds check for values < 0.
pub fn factorial_1(n: i32) -> i32 {
    if n < 0 {
        println!("Error: Cannot compute factorials for n < 0");
        return n;
    } else if n == 0 {
        return 1;
    } else {
        return n * factorial_1(n - 1);
    }
}
// After analyzing the book's example (cheating)
/** Recursive implementation of a factorial calculator up to 12! in O(n) time */
pub fn factorial_2(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    return n * factorial_2(n - 1);
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
// Reimplementation as an iterative function for a more logical evaluation (cheat)
/** Iterative implementation of a factorial calculator up to 12! in O(n) time */
pub fn factorial_4(n: u32) -> u32 {
    let mut fac = 1;
    for e in 2..=n {
        fac *= e
    }
    fac
}

/** Recursive implementation of a binary search in O(log n) time.
 * Returns the index of the target within a given array, if present.
 * Otherwise the function returns -1. */
pub fn bin_search_0(a: &Vec<i32>, t: i32, left: i32, right: i32) -> i32 {
    // Recursive base case
    if left > right {
        return -1;
    } else {
        let mid = (left + right) / 2;
        if t == a[mid as usize] {
            return mid as i32;
        } else if t < a[mid as usize] {
            return bin_search_0(&a, t, left, mid - 1);
        } else {
            return bin_search_0(&a, t, mid + 1, right);
        }
    }
}
pub fn bin_search_0_wrapper(a: &Vec<i32>, target: i32) -> i32 {
    let right = a.clone().len() as i32 - 1;
    return bin_search_0(a, target, 0, right);
}
// The recursive binary search algorithm represents tail recursion.
// Even though Rust (likely) reimplements this automatically this is done
// manually as a fun exercise.
/** Iterative reimplementation of a recursive binary search algorithm in O(log n) time.
 * This algorithm takes a referenced vec and returns the index of the target, if it exists.
 * Otherwise it returns -1 to indicate that the target is not in the vec. */
pub fn bin_search_1(data: &Vec<i32>, target: i32) -> i32 {
    let mut low = 0;
    let mut high = data.clone().len() - 1;
    while low <= high {
        let mid: usize = (low + high) / 2;
        if target == data[mid] {
            return mid as i32;
        } else if target < data[mid] {
            high = mid - 1
        } else {
            low = mid + 1
        }
    }
    return -1;
}
#[test]
pub fn bin_search_test() {
    let v = vec![12, 26, 31, 48, 52, 61, 75, 80, 93];
    assert_eq!(2, bin_search_0_wrapper(&v, 31));
    assert_eq!(6, bin_search_1(&v, 75));
}

// Initially it appears this algorithm runs in O(n^2) time, but it actually
// runs in O(n) time because it touches (and performs O(1) operations) on
// n nodes in the tree exactly once. This algorithm represents multiple recursion
// because for each invocation there are x number of directory nodes to sum.
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
        let this_dir = std::fs::metadata(root)
            .expect("metadata call failed [0]")
            .len();
        //dir_size += this_dir;
        println!("d {:>7}B  {}", dir_size + this_dir, root.display());
    } else if root.is_file() {
        let size = std::fs::metadata(root)
            .expect("metadata call failed [1]")
            .len();
        println!("  {:>7}B  {}", size, root.display());
        return size;
    }
    return dir_size;
}

// Sum of array of integers to n indexes in O(n) time using linear recursion
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
pub fn array_sum_2(data: &Vec<i32>, n: usize) -> i32 {
    if n == 0 {
        return data[0];
    } else {
        return array_sum_2(&data, n - 1) + &data[n];
    }
}
// Sum of an array of integers to n in O(n) time using O(log n) space with binary recursion
pub fn array_sum_4(data: Vec<i32>) -> i32 {
    let h = data.clone().len() - 1;
    fn array_sum_3(data: Vec<i32>, low: usize, high: usize) -> i32 {
        if low > high {
            0
        } else if low == high {
            return data[low];
        } else {
            let mid = (low + high) / 2;
            return array_sum_3(data.clone(), low, mid) + array_sum_3(data, mid + 1, high);
        }
    }
    return array_sum_3(data, 0, h);
}
// Represents a public interface for the unsightly recursive implementation in array_sum_3()
//pub fn array_sum_4(data: Vec<i32>) -> i32 {
//    let h = data.clone().len() - 1;
//    return array_sum_3(data, 0, h)
//}
#[test]
pub fn array_sum_test() {
    // Iterative impelmentation test
    let t = vec![6, 7, 8, 9];
    assert_eq!(30, array_sum_0(t));
    // Recursive implementation test
    // Dumb, error prone implementation
    let t = vec![6, 7, 8, 9];
    assert_eq!(30, array_sum_2(&t, 3));

    // Binary recursive implementation test
    let t = vec![6, 7, 8, 9];
    assert_eq!(30, array_sum_4(t));
}

// Iteratively reverses the elements of an array
/** Iteratively reverses the elements of an array in O(n) time */
pub fn array_reversal_0(mut v: Vec<i32>) -> Vec<i32> {
    let mut high = v.len() - 1; // Match the index values, not # of elements
    let mut low = 0;
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
// Linear recursion to reverse the elements of an array in place
// (with liberal type conversion)
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

// Computing powers
// First attempt uses iteration
pub fn powers_0(x: u32, n: u32) -> u32 {
    let mut product = 1;
    for _ in 1..=n {
        product *= x;
    }
    product
}
// My attempt at linear recursion used to compute powers
pub fn powers_1(x: u32, mut product: u32, n: u32) -> u32 {
    if n > 1 {
        product *= x;
        return powers_1(x, product, n - 1);
    }
    return product;
}
// Transling the book's Java example that runs in O(n) time
pub fn powers_2(x: u32, n: u32) -> u32 {
    if n > 1 {
        return x * powers_2(x, n - 1);
    }
    return x;
}
// Reimplementation using binary recursion that computes powers in O(log n) time
// Relies on truncation in integer division; if n is even the result is x^n, and
// if n is even the result is x^(n-1) * x
pub fn powers_3(x: u32, n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        let partial = powers_3(x, n / 2); // Relies on truncation
        let mut result = partial * partial;
        if n % 2 == 1 {
            result *= x
        }
        result
    }
}
#[test]
pub fn powers_test() {
    assert_eq!(128, powers_0(2, 7));
    assert_eq!(0, powers_0(0, 7));
    assert_eq!(1, powers_0(1, 7));
    assert_eq!(128, powers_1(2, 2, 7));
    assert_eq!(0, powers_1(0, 0, 7));
    assert_eq!(1, powers_1(1, 1, 7));
    assert_eq!(256, powers_3(2, 8));
    assert_eq!(128, powers_3(2, 7));
    assert_eq!(0, powers_3(0, 7));
    assert_eq!(1, powers_3(1, 7));
    assert_eq!(19683, powers_3(3, 9));
}

// Another stab at Fibonacci sequence
pub fn fib_0(n: i32) -> Vec<i32> {
    let mut seq = Vec::new();
    let mut first = 0;
    let mut next = 1;
    let mut this: i32;
    seq.push(first);
    seq.push(next);
    for _ in 2..n {
        this = first + next;
        seq.push(this);
        first = next;
        next = this;
    }
    seq
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
