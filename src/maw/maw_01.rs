/*!
This is a sandbox crate for chapter 1 of Data Structures and Algorithm Analysis in Java by Mark Allen Weiss
*/

pub fn recursion(n: i32) {
    // Defines base case
    if n >= 10 {
        // Recursive call to self
        recursion(n / 10);
    }
    // Prints the digit
    println!("{}", n % 10)
}

/** My (iterative) version of a binary search implementation;
 * Takes a sorted array and a key and returns either Some(index) or None */
pub fn binary_search(a: &[i32], key: i32) -> Option<i32> {
    use std::cmp::Ordering;

    // Sets initial position of the search boundaries
    let mut left = 0;
    let mut right = a.len() - 1;

    // Loops until the search boundaries overlap;
    // If the loop doesn't find the key, the function
    // returns None
    while left <= right {
        let mid = (left + right) / 2;
        match a[mid].cmp(&key) {
            Ordering::Equal => return Some(mid as i32),
            Ordering::Greater => right = mid - 1,
            Ordering::Less => left = mid + 1,
        }
        //if a[mid] == key {
        //    return Some(mid as i32);
        //} else if a[mid] > key {
        //    right = mid - 1;
        //} else {
        //    left = mid + 1;
        //}
        //match key {
        //    val if val == a[mid] => Some(mid as i32),
        //    _ => Some(0)
        //};
        println!("Guess index: {}", &mid);
    }
    None
}

#[test]
pub fn binary_search_test() {
    // The target 73 exists at the 37th index
    let target = 73;
    let array: [i32; 39] = [
        1, 4, 5, 6, 10, 12, 16, 21, 23, 24, 25, 27, 31, 32, 33, 35, 37, 39, 40, 41, 42, 43, 45, 47,
        49, 50, 51, 52, 54, 56, 57, 60, 61, 67, 70, 71, 72, 73, 74,
    ];
    let result = binary_search(&array, target).unwrap_or_default();
    assert_eq!(result, 37)
}

use std::collections::HashMap;

fn longest_unique_substring(s: &str) -> usize {
    let mut map = HashMap::new(); // As HashMap<char, i32>
    let mut left = 0;
    let mut max_len = 0;

    let chars: Vec<char> = s.chars().collect();

    for right in 0..chars.len() {
        *map.entry(chars[right]).or_insert(0) += 1;

        while map[&chars[right]] > 1 {
            *map.get_mut(&chars[left]).unwrap() -= 1;
            left += 1;
        }

        max_len = max_len.max(right - left + 1);
    }

    max_len
}

fn longest_unique_substring_print(s: &str) -> usize {
    let mut map = HashMap::new();
    let mut left = 0;
    let mut max_len = 0;
    let mut best_range = (0, 0); // To store (start, end)

    let chars: Vec<char> = s.chars().collect();

    for right in 0..chars.len() {
        *map.entry(chars[right]).or_insert(0) += 1;

        while map[&chars[right]] > 1 {
            *map.get_mut(&chars[left]).unwrap() -= 1;
            left += 1;
        }

        let current_len = right - left + 1;
        if current_len > max_len {
            max_len = current_len;
            best_range = (left, right);
        }
    }

    // Print the substring using the captured indices
    let result: String = chars[best_range.0..=best_range.1].iter().collect();
    println!("Longest substring found: \"{result}\" (length {max_len})");

    max_len
}

#[test]
fn longest_unique_substring_test() {
    let s = "this is a llama test.";
    assert_eq!(longest_unique_substring(s), 6);
    longest_unique_substring_print(s);
    //panic!()
}
