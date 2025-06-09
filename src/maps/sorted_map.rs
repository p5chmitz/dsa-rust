/*! Safe sorted map using Vec

# About

 ```rust
    use dsa_rust::maps::sorted_map::SortedMap;

    // Establishes a baseline map
    let list = vec![
        ("Bobson", 29),
        ("Brain", 39),
        ("Dangus", 34),
        ("Dingus", 34),
        ("Peter", 41),
        ("Pingus", 45),
        ("Remus", 27),
        ("Romulus", 28),
    ];
    let mut map = SortedMap::new();

    assert_eq!(map.cap(), 0);
    assert_eq!(map.entries(), 0);
    for e in list {
        eprintln!("Inserting {}", &e.0);
        map.put(e.0, e.1);
    }
    assert_eq!(map.cap(), 8);
    assert_eq!(map.entries(), 8);

    // Tests the get function
    let age = map.get("Peter");
    assert!(age.is_some());
    assert_eq!(age.unwrap(), &41);

    // Tests replacing a value for an existing key
    let old = map.put("Peter", 42);
    assert_eq!(*old.unwrap().value(), 41);
    let age = map.get("Peter");
    assert!(age.is_some());
    assert_eq!(age.unwrap(), &42);
    assert_eq!(map.entries(), 8);

    // Tests the remove function
    let removed = map.remove("Pingus");
    assert!(removed.is_some());
    assert_eq!(*removed.unwrap().value(), 45);
    assert_eq!(map.entries(), 7);

 ```
*/

//use std::cmp::Ordering::{Equal, Greater, Less};

use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct Entry<K, V> {
    key: K,
    value: V,
}
impl<K, V> Entry<K, V>
where
    K: Debug + PartialEq,
    V: PartialEq,
{
    /// Constructs a new Entry
    fn new(key: K, value: V) -> Entry<K, V> {
        Entry { key, value }
    }

    /// Returns the key for an Entry
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Returns the value for an Entry
    pub fn value(&self) -> &V {
        &self.value
    }
}
#[derive(Debug)]
pub struct SortedMap<K, V> {
    data: Vec<Option<Entry<K, V>>>,
    size: usize,
}
impl<K, V> Default for SortedMap<K, V>
where
    K: Debug + PartialEq + Ord,
    V: PartialEq,
 {
    fn default() -> Self {
        Self::new()
    }
}
impl<K, V> SortedMap<K, V>
where
    K: Debug + PartialEq + Ord,
    V: PartialEq,
{

    /** Every structure needs a constructor */
    pub fn new() -> SortedMap<K, V> {
        SortedMap {
            data: Vec::new(),
            size: 0,
        }
    }

    /// Returns the capacity of the map
    pub fn cap(&self) -> usize {
        self.data.len()
    }

    /// Returns the number of entries in the map
    pub fn entries(&self) -> usize {
        self.size
    }

    /** Convenience wrapper for busy recursive signature method that does all the real work;
    NOTE: Calling function must handle case of entering first entry into empty list */
    fn find_index(&self, target: &K) -> usize {
        let right: usize = self.data.len() - 1;
        Self::find_index_rec(&self.data, target, 0, right)
    }
    /** If the tarket is in the list the function returns the index,
    if the target is not in the list the function returns the next appropriate index */
    fn find_index_rec(
        data: &Vec<Option<Entry<K, V>>>,
        key: &K,
        left: usize,
        right: usize,
    ) -> usize {
        // Recursive base case returns next viable index when the key is not found
        if left > right {
            left
        } else {
            let mid = (left + right) / 2;
            if data[mid].is_some() && *key == data[mid].as_ref().unwrap().key {
                mid
            } else if data[mid].is_some() && *key < data[mid].as_ref().unwrap().key {
                Self::find_index_rec(data, key, left, mid - 1)
            } else {
                Self::find_index_rec(data, key, mid + 1, right)
            }
        }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let j: usize = self.find_index(&key);
        if self.data[j].is_some() && self.data[j].as_ref().unwrap().key == key {
            return Some(&self.data[j].as_ref().unwrap().value);
        }
        None
    }

    /** Inserts an entry into the map while maintaining sorted order by key */
    pub fn put(&mut self, key: K, value: V) -> Option<Entry<K, V>> {
        // First entry
        if self.data.is_empty() {
            let new: Entry<K, V> = Entry::new(key, value);
            self.data.insert(0, Some(new));
            self.size += 1;
            return None;
        }

        // Searches for existing entry
        let index = self.find_index(&key);
        // The key already exists
        if index < self.data.len() && self.data[index].as_ref().unwrap().key == key {
            // The location has a previous value
            let old: Entry<K, V> = self.data[index].take().unwrap();
            let new: Entry<K, V> = Entry::new(key, value);
            self.data[index] = Some(new);
            Some(old)
        } else {
            let new: Entry<K, V> = Entry::new(key, value);
            self.data.insert(index, Some(new));
            self.size += 1;
            None
        }
    }

    pub fn remove(&mut self, key: K) -> Option<Entry<K, V>> {
        let j: usize = self.find_index(&key);
        if self.data[j].is_some() && self.data[j].as_ref().unwrap().key == key {
            self.size -= 1;
            return self.data[j].take();
        }
        None
    }
}

#[test]
fn find_index_test() {
    let list = vec![
        Some(Entry {
            key: "Bobson",
            value: 29,
        }),
        Some(Entry {
            key: "Brain",
            value: 39,
        }),
        Some(Entry {
            key: "Dangus",
            value: 34,
        }),
        Some(Entry {
            key: "Dingus",
            value: 34,
        }),
        Some(Entry {
            key: "Peter",
            value: 41,
        }),
        Some(Entry {
            key: "Pingus",
            value: 45,
        }),
        Some(Entry {
            key: "Remus",
            value: 27,
        }),
        Some(Entry {
            key: "Romulus",
            value: 28,
        }),
    ];
    // Key exists
    let index = SortedMap::find_index_rec(&list, &"Brain", 0, list.len() - 1);
    assert_eq!(index, 1);
    // Key exists
    let index = SortedMap::find_index_rec(&list, &"Pingus", 0, list.len() - 1);
    assert_eq!(index, 5);
    // Key does not exist
    let index = SortedMap::find_index_rec(&list, &"Florbus", 0, list.len() - 1);
    assert_eq!(index, 4);
}

#[test]
fn general_test() {
    // Establishes a baseline map
    let list = vec![
        ("Bobson", 29),
        ("Brain", 39),
        ("Dangus", 34),
        ("Dingus", 34),
        ("Peter", 41),
        ("Pingus", 45),
        ("Remus", 27),
        ("Romulus", 28),
    ];
    let mut map = SortedMap::new();
    assert_eq!(map.data.len(), 0);
    assert_eq!(map.size, 0);
    for e in list {
        eprintln!("Inserting {}", &e.0);
        map.put(e.0, e.1);
    }
    assert_eq!(map.data.len(), 8);
    assert_eq!(map.size, 8);

    // Tests the get function
    let age = map.get("Peter");
    assert!(age.is_some());
    assert_eq!(age.unwrap(), &41);

    // Tests replacing a value for an existing key
    let old = map.put("Peter", 42);
    assert_eq!(old.unwrap().value, 41);
    let age = map.get("Peter");
    assert!(age.is_some());
    assert_eq!(age.unwrap(), &42);
    assert_eq!(map.size, 8);

    // Tests the remove function
    let removed = map.remove("Pingus");
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().value, 45);
    assert_eq!(map.size, 7);
}
