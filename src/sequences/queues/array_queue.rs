// This module didn't make the cut (obviously):
// Suppress unused code warnings
#![allow(dead_code)]

pub struct CircQueue<T> {
    data: [T; 100],
    front: usize,
    back: usize,
    size: usize,
    capacity: usize,
}
