/*!
This is a sandbox crate for chapter 1 of Data Structures and Algorithm Analysis in Java by Mark Allen Weiss
*/

#![allow(dead_code)]

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

fn insertion_sort<T: Ord>(list: &mut [T]) {
    // operates on 2nd element through n - 1 because .. is
    // upper bounds exclusive
    for i in 1..list.len() {
        let mut j = i; // Required because i is not mutable
        while j > 0 && list[j] < list[j - 1] {
            list.swap(j, j - 1);
            j -= 1;
        }
    }
}
fn insertion_sort2<T: Ord>(list: &mut [T]) {
    for mut i in 1..list.len() {
        while i > 0 && list[i] < list[i - 1] {
            list.swap(i, i - 1);
            i -= 1;
        }
    }
}
#[test]
fn insertion_sort_test() {
    let mut a = [2, 5, 8, 3, 5, 9];
    insertion_sort(&mut a);
    assert_eq!(a, [2, 3, 5, 5, 8, 9]);

    let mut a = [2, 5, 8, 3, 5, 9];
    insertion_sort2(&mut a);
    assert_eq!(a, [2, 3, 5, 5, 8, 9]);
}

use std::ptr;
fn unsafe_insertion_sort<T: Ord>(list: &mut [T]) {
    for i in 1..list.len() {
        unsafe {
            // Move the element out without dropping it
            let tmp = ptr::read(&list[i]);
            let mut j = i;

            // Shift elements right until correct spot is found
            while j > 0 && tmp < list[j - 1] {
                let dst = list.as_mut_ptr().add(j);
                let src = list.as_ptr().add(j - 1);
                ptr::copy_nonoverlapping(src, dst, 1);
                j -= 1;
            }

            // Place the element into its final position
            ptr::write(list.as_mut_ptr().add(j), tmp);
        }
    }
}

fn selection_sort<T: Ord>(list: &mut [T]) {
    let len = list.len();

    for i in 0..len {
        let mut min_idx = i;

        for j in (i + 1)..len {
            if list[j] < list[min_idx] {
                min_idx = j;
            }
        }

        if min_idx != i {
            list.swap(i, min_idx);
        }
    }
}

fn unsafe_selection_sort<T: Ord>(list: &mut [T]) {
    let len = list.len();
    let ptr = list.as_mut_ptr();

    for i in 0..len {
        unsafe {
            let mut min_idx = i;

            for j in (i + 1)..len {
                if (*ptr.add(j)) < (*ptr.add(min_idx)) {
                    min_idx = j;
                }
            }

            if min_idx != i {
                ptr::swap(ptr.add(i), ptr.add(min_idx));
            }
        }
    }
}

fn merge_sort<T: Ord>(list: &mut [T]) {
    let len = list.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let (left, right) = list.split_at_mut(mid);

    merge_sort(left);
    merge_sort(right);

    // Allocate temporary buffer
    let mut buf: Vec<T> = Vec::with_capacity(len);
    let _: &mut [std::mem::MaybeUninit<T>] = buf.spare_capacity_mut();

    unsafe {
        // Set length without initializing
        buf.set_len(len);

        merge_into(left, right, &mut buf);

        // Move merged result back into original slice
        ptr::copy_nonoverlapping(buf.as_ptr(), list.as_mut_ptr(), len);
    }
}

unsafe fn merge_into<T: Ord>(left: &mut [T], right: &mut [T], buf: &mut [T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            ptr::write(&mut buf[k], ptr::read(&left[i]));
            i += 1;
        } else {
            ptr::write(&mut buf[k], ptr::read(&right[j]));
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        ptr::write(&mut buf[k], ptr::read(&left[i]));
        i += 1;
        k += 1;
    }

    while j < right.len() {
        ptr::write(&mut buf[k], ptr::read(&right[j]));
        j += 1;
        k += 1;
    }
}

fn quick_sort<T: Ord>(list: &mut [T]) {
    if list.len() <= 1 {
        return;
    }

    let pivot_index = partition(list);

    let (left, right) = list.split_at_mut(pivot_index);
    quick_sort(left);
    quick_sort(&mut right[1..]); // skip pivot
}

fn partition<T: Ord>(list: &mut [T]) -> usize {
    let len = list.len();
    let pivot_index = len - 1;

    let mut store = 0;

    for i in 0..pivot_index {
        if list[i] < list[pivot_index] {
            list.swap(i, store);
            store += 1;
        }
    }

    list.swap(store, pivot_index);
    store
}
