#![allow(dead_code)]
#![allow(unused)]

/** Skip lists */
// Skip list map
// Vec<LinkedList<Entry<K, V>>>
use std::fmt::Debug;

#[derive(Debug)]
pub struct Entry<K, V> {
    key: K,
    value: V,
}
impl<K, V> Entry<K, V>
where
    K: Debug + PartialEq,
    V: PartialEq,
{
    fn new(key: K, value: V) -> Entry<K, V> {
        Entry { key, value }
    }
}
#[derive(Debug)]
pub struct SortedMap<K, V> {
    data: Vec<Vec<Option<Entry<K, V>>>>,
    height: usize,
}
impl<K, V> SortedMap<K, V>
where
    K: Debug + PartialEq + Ord,
    V: PartialEq,
{
}
