/*! A proper sorted map

# About
This sorted map uses the library's [AVL tree]() as its backing structure, providing _O(log(n))_ search, insert, and delete operations.

# Example

```rust
    use dsa_rust::associative::avl_tree_map::TreeMap;

    let text = "and the final paragraph clearly came from the heart, 
    or whatever cool yet sensitive organ Sadie kept in place of one.";

    let mut map = TreeMap::<char, usize>::new();

    for e in text.chars() {
        if map.contains(e) {
            let old = map.remove(e).unwrap();
            map.put(e, old.value() + 1);
        } else {
            map.put(e, 1);
        }
    }
    println!("TreeMap character occurrence");
    for e in map.iter() {
        println!("{e:?}");
    }

    println!("\nTreeMap vowel occurrence");
    for vowel in ['a', 'e', 'i', 'o', 'u', 'y'] {
        eprintln!("{vowel}: {}", map.get(vowel).unwrap_or(&0));
    }

```

```text
TreeMap character occurrence
('\n', 1)
(' ', 24)
(',', 1)
('.', 1)
('S', 1)
('a', 12)
('c', 4)
('d', 2)
('e', 14)
('f', 3)
('g', 2)
('h', 5)
('i', 5)
('k', 1)
('l', 5)
('m', 2)
('n', 6)
('o', 7)
('p', 4)
('r', 8)
('s', 2)
('t', 7)
('v', 2)
('w', 1)
('y', 2)

TreeMap vowel occurrence
a: 12
e: 14
i: 5
o: 7
u: 0
y: 2
```

*/

use crate::hierarchies::avl_tree::AVLTree;

use std::fmt::Debug;
use std::cmp::Ordering;
use std::borrow::Borrow;

/// The wrapper struct that allows TreeMap<K, V> to use AVLTree<T>. 
/// Because `T` is [Ord], Entry<K, V> must implement [Eq] and [PartialOrd], 
/// which themselves must implement [PartialEq]. 
/// All traits use `key` for ordering.
/// 
/// See the [module-level documentation]() for more details.
#[derive(Debug)]
pub struct Entry<K, V> {
    key: K,
    value: V,
}
impl<K, V> Entry<K, V> {
    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn value(&self) -> &V {
        &self.value
    }
}
impl<K: PartialEq, V> PartialEq for Entry<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
// Eq requires PartialEq
impl<K: Eq, V> Eq for Entry<K, V> {}
// PartialOrd requires PartialEq
impl<K: PartialOrd, V> PartialOrd for Entry<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}
// Ord requires Eq + PartialOrd
impl<K: Ord, V> Ord for Entry<K, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}
impl<K, V> Borrow<K> for Entry<K, V> {
    fn borrow(&self) -> &K {
        &self.key
    }
}

/// 
/// 
/// See the [module-level documentation]() for more details.
#[derive(Debug)]
pub struct TreeMap<K, V> {
    tree: AVLTree<Entry<K, V>>,
    size: usize
}
impl<K, V> Default for TreeMap<K, V>
where
    K: Default + Eq + Ord + PartialEq,
 {
    fn default() -> Self {
        Self::new()
    }
}
impl<K, V> TreeMap<K, V> 
where
    K: Default + Eq + Ord + PartialEq,
{
    /// Constructor
    pub fn new() -> Self {
        Self {
            tree: AVLTree::<Entry<K, V>>::new(),
            size: 0
        }
    }

    /// Returns the number of elements in the map.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns `true` if the map contains an entry associated with the given key.
    pub fn contains(&self, key: K) -> bool {
        self.tree.contains(&key)
    }

    /// Returns the value associated with the key, if `Some`.
    pub fn get(&self, key: K) -> Option<&V> {
        if let Some(val) = self.tree.get_node(&key) {
            return Some(&val.value)
        } None
    }

    /// Inserts the entry into the map. If the key already exists, removes
    /// and returns the old entry and inserts a new one. This allows for
    /// situations where keys are `==` without being _identical_.
    pub fn put(&mut self, key: K, value: V) -> Option<Entry<K, V>> {
        let new_entry = Entry { key, value };
        let old_entry = self.tree.remove(&new_entry.key);
        self.tree.insert(new_entry);
        if old_entry.is_none() {
            self.size += 1;
        }
        old_entry
    }

    /// Mutates
    pub fn mut_val_or() {}

    /// Removes and returns the entry associated with the key:value pair, 
    /// if it exists in the map.
    pub fn remove(&mut self, key: K) -> Option<Entry<K, V>> {
        if self.tree.contains(&key) {
            self.size -= 1;
            return self.tree.remove(&key)
        } None
    }

    /// Returns an iterator over borrowed values. The resulting values
    /// appear in sorted order.
    ///
    /// Example use:
    /// ```rust
    /// use dsa_rust::associative::probing_hash_table::HashMap;
    /// let mut count: HashMap<char, u8> = HashMap::new();
    /// let mut v = Vec::new();
    /// for e in count.iter() {
    ///     v.push(*e.0);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            iter: self.tree.iter(),
        }
    }

}

pub struct Iter<'a, K, V> {
    //iter: std::slice::Iter<'a, Option<Entry<K, V>>>,
    iter: crate::hierarchies::avl_tree::InOrderIter<'a, crate::associative::avl_tree_map::Entry<K, V>>
}
impl<'a, K, V> Iterator for Iter<'a, K, V>
where
    K: Debug + PartialEq,
    V: PartialEq,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|entry| (&entry.key, &entry.value))
    }
}

#[test]
// Generic type test
fn avl_tree_map_test() {
    //Creates a new hash map
    let mut map = TreeMap::<&str, u8>::new();

    assert_eq!(map.size(), 0);

    // Illustrates that put() and get() work
    map.put("Peter", 40);
    assert_eq!(map.size(), 1);

    // Illustrates that the map grows correctly
    map.put("Brain", 39); // Grows the map
    map.put("Remus", 22);
    map.put("Bobson", 36); // Grows the map
    map.put("Dingus", 18);
    map.put("Dangus", 27); // Grows the map
    assert_eq!(map.size(), 6);

    // Underlying tree arena check
    eprintln!("Initial state");
    assert_eq!(map.tree.arena.len(), 6);
    for e in map.tree.arena.iter() {
        eprintln!("{e:?}")
    }

    // Illustrates that contains() works as intended
    assert!(map.contains("Dingus"));

    // Illustrates that put() returns old values and
    // overwrites existing values upon collision...
    let old = map.put("Peter", 41).unwrap();
    assert_eq!(old.value, 40_u8);
    let new_val = map.get("Peter").unwrap();
    assert_eq!(*new_val, 41);
    assert_eq!(map.size(), 6);

    // Underlying tree arena check
    eprintln!("After replacement");
    assert_eq!(map.tree.arena.len(), 7);
    for e in map.tree.arena.iter() {
        eprintln!("{e:?}")
    }

    // Illustrates that removes entries by key and returns the value
    assert!(map.contains("Dangus"));
    let removed = map.remove("Dangus").unwrap();
    assert_eq!(map.size(), 5);

    // Underlying tree arena check
    eprintln!("After removal");
    assert_eq!(map.tree.arena.len(), 7);
    for e in map.tree.arena.iter() {
        eprintln!("{e:?}")
    }

    //assert_eq!(removed.key(), &"Dangus");
    //assert_eq!(removed.value(), &27);
    assert_eq!(removed.key, "Dangus");
    assert_eq!(removed.value, 27);
    assert!(!map.contains("Dangus"));

    eprintln!("{map:#?}");

    //panic!("MANUAL TEST FAILURE");
}
//
//#[test]
// //Tests the custom update value function
//fn mut_val_test() {
//    //Creates a new hash map
//    let mut count = HashMap::<char, u8>::new();
//    let phrase: &str = "Hello, sickos";
//
//    // Seeds the map with just the characters and a default value
//    for char in phrase.chars() {
//        count.put(char, 0);
//    }
//
//    eprintln!("\nInitial state:");
//    count.contents();
//
//    // Iterates through the map again, updating each value based on its occurence
//    // NOTE: Can also be used as initial mapping operation with the same code, 
//    // but was split here for illustrative purposes
//    for char in phrase.chars() {
//        count.mut_val_or(char, |x| *x += 1, 1);
//    }
//
//    // Pretty-prints the contents of the map
//    eprintln!("\nModified:");
//    count.contents();
//
//    // Uncomment to trigger debug print
//    //assert!(count.is_empty());
//}
//
#[test]
// Tests that the structure is iterable
fn iter_test() {
    //Creates a new hash map of the frequence letters in the given phrase
    let mut map = TreeMap::<usize, char>::new();
    for (index, char) in "acbjfed".chars().enumerate() {
        map.put(index, char); // index is key, char is value
    }

    // Indicates that the map is indeed sorted
    eprintln!("\nSorted map:");
    for e in map.tree.iter() {
        eprintln!("{e:?}")
    }

    // Prints only the middle values based on key range
    // which should be "b, j, f, e"
    eprintln!("\nPrint values for indexes 2..=5:");
    for e in 2..=5 {
        let val = map.get(e).unwrap();
        eprintln!("{val:?}");
    }

    //let text = "Not the most elegant text, but it sure 
    //    does the trick when you need it. I could go on and on and on, 
    //but instead Id rather just let the text explain itself in order to Illustrate
    //    what this test is all about. Then again Im not sure I could do this without
    //    just a little rambling, amirite? You're the kind of person that
    //    gets it, I think.";

    let text = "and the final paragraph clearly came from the heart, 
    or whatever cool yet sensitive organ Sadie kept in place of one.";

    // Establishes parity with the std BTreeMap
    //let mut map = std::collections::BTreeMap::<char, usize>::new();
    //for e in text.chars() {
    //    if map.contains_key(&e) {
    //        let old = map.remove(&e).unwrap();
    //        map.insert(e, old + 1);
    //    } else {
    //        map.insert(e, 1);
    //    }
    //}
    //eprintln!("\nBTreeMap character occurrence");
    //for e in map.iter() {
    //    eprintln!("{e:?}");
    //}

    // Second attempt with custom TreeMap
    let mut map = TreeMap::<char, usize>::new();
    for e in text.chars() {
        if map.contains(e) {
            let old = map.remove(e).unwrap();
            map.put(e, old.value + 1);
        } else {
            map.put(e, 1);
        }
    }
    eprintln!("\nTreeMap character occurrence");

    //for e in map.tree.iter() {
    //    eprintln!("({:?}, {})", e.key, e.value);
    //}
    for e in map.iter() {
        eprintln!("{e:?}");
    }

    eprintln!("\nTreeMap vowel occurrence");
    for vowel in ['a', 'e', 'i', 'o', 'u', 'y'] {
        eprintln!("{vowel}: {}", map.get(vowel).unwrap_or(&0));
    }
    eprintln!("\nTreeMap occurrence of characters in my name");
    for vowel in ['p', 'e', 't', 'r'] {
        eprintln!("{vowel}: {}", map.get(vowel).unwrap_or(&0));
    }

    // Uncomment to trigger debug print
    panic!("MANUAL TEST FAILURE");
}
