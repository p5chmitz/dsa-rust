pub struct CircQueue<T> {
    data: [T; 100],
    front: usize,
    back: usize,
    size: usize,
    capacity: usize,
}
