#![allow(clippy::uninlined_format_args)] // Cant inline all args in print

/*! A set structure based on this library's HashMap

# About
This simple set structure offers mutating and non-mutating versions of basic set operations. Currently, the structure contains the following operations: 
- [Union](crate::associative::hash_set::HashSet::union): Returns an iterator over references to all elements that appear in either `self` or `other` as a set-theoretic [union](https://en.wikipedia.org/wiki/Union_(set_theory)).
- [Intersection](crate::associative::hash_set::HashSet::intersection): Returns an iterator over references to elements that appear in both `self` and `other` as a set-theoretic [intersection](https://en.wikipedia.org/wiki/Intersection_(set_theory)).
- [Retain All](crate::associative::hash_set::HashSet::retain_all): Mutates self such that self only keeps elements that are also in other. This function could also appropriately be called "intersect", but that was too close to the non-mutating version of this function called "intersection". 
- [Difference](crate::associative::hash_set::HashSet::difference): Returns an iterator over references to elements that are present in `self` but not in `other` as a set-theoretic asymmetric difference or [relative complement](https://en.wikipedia.org/wiki/Complement_(set_theory)). This operation is asymmetric: interchanging self and other generally produces different results.
- [Unique](crate::associative::hash_set::HashSet::unique): Returns an iterator over references to elements that are present in exactly one of the two sets as a set-theoretic [symmetric difference](https://en.wikipedia.org/wiki/Symmetric_difference). This operation is symmetric: interchanging self and other yields the same result.

# Design
This structure is built using the library's custom [hash map](crate::associative::probing_hash_table). This structure represents mostly an exercise in iterator implementation.

# Example

```rust
use dsa_rust::associative::hash_set::HashSet;

    //Creates a couple new hash sets
    let mut set1 = HashSet::<u8>::new();
    // [0, 1, 2, 3]
    for n in 0..=3 {
        set1.put(n); 
    }

    let mut set2 = HashSet::<u8>::new();
    // [2, 3, 4, 5]
    for n in 2..=5 {
        set2.put(n); 
    }

    // Creates a list of all elements in both self and other
    let mut union: Vec<u8> = Vec::new();
    for e in set1.union(&set2) {
        union.push(*e); 
    }
    assert_eq!(union.len(), 6);
    assert!(union.contains(&0));
    assert!(union.contains(&1));
    assert!(union.contains(&2));
    assert!(union.contains(&3));
    assert!(union.contains(&4));
    assert!(union.contains(&5));

    // Creates a list of elements only present in both sets
    let mut intersection: Vec<u8> = Vec::new();
    for e in set1.intersection(&set2) {
        intersection.push(*e); 
    }
    assert_eq!(intersection.len(), 2);
    assert!(intersection.contains(&2));
    assert!(intersection.contains(&3));

    // Creates a list of elements from self not present in other
    let mut difference: Vec<u8> = Vec::new();
    for e in set1.difference(&set2) {
        difference.push(*e); 
    }
    assert_eq!(difference.len(), 2);
    assert!(difference.contains(&0));
    assert!(difference.contains(&1));

    // Proves that difference() is asymmetric 
    let mut difference: Vec<u8> = Vec::new();
    for e in set2.difference(&set1) {
        difference.push(*e); 
    }
    assert_eq!(difference.len(), 2);
    assert!(difference.contains(&4));
    assert!(difference.contains(&5));

    // Creates a list of elements present in either the first or 
    // second one set, but not both
    let mut unique: Vec<u8> = Vec::new();
    for e in set1.unique(&set2) {
        unique.push(*e); 
    }
    assert_eq!(unique.len(), 4);
    assert!(unique.contains(&0));
    assert!(unique.contains(&1));
    assert!(unique.contains(&4));
    assert!(unique.contains(&5));

    // Illustrates that unique() is symmetric
    let mut unique: Vec<u8> = Vec::new();
    for e in set2.unique(&set1) {
        unique.push(*e); 
    }
    assert_eq!(unique.len(), 4);
    assert!(unique.contains(&0));
    assert!(unique.contains(&1));
    assert!(unique.contains(&4));
    assert!(unique.contains(&5));
```

*/

use crate::associative::probing_hash_table;

use std::fmt::Debug;
use std::hash::Hash;

pub struct HashSet<T> {
    map: probing_hash_table::HashMap<T, ()>
}
impl<T> Default for HashSet<T>
where
    T: Clone + Default + Debug + Hash + Eq + PartialEq,
 {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> HashSet<T>
where
    T: Clone + Default + Debug + Hash + Eq + PartialEq, {

    // Basic operations
    ///////////////////

    /// Constructor for an empty map with a default capacity of 2.
    pub fn new() -> Self {
        let new_capacity = 2;
        Self {
            map: probing_hash_table::HashMap::<T, ()>::new_with_capacity(new_capacity),
        }
    }

    /// Constructor for an empty map with a given capacity.
    pub fn new_with_capacity(size: usize) -> Self {
        Self {
            map: probing_hash_table::HashMap::<T, ()>::new_with_capacity(size),
        }
    }

    /// Returns the number of elements in the set.
    pub fn size(&self) -> usize {
        self.map.len()
    }

    /// Returns true if the set is either empty or contains only empty slots and deleted entries.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Adds an item to the set.
    pub fn put(&mut self, elem: T) {
        self.map.put(elem, ());
    }

    /// Removes an item from the set.
    pub fn remove(&mut self, elem: T) -> Option<T> {
        if let Some(val) = self.map.remove(&elem) {
            return Some(val.key().clone());
        } None
    }

    /// Returns true if the set contains the referenced element.
    pub fn contains<Q>(&self, elem: &Q) -> bool
    where
        T: std::borrow::Borrow<Q>,
        Q: Debug + Hash + Eq + ?Sized,
    {
        self.map.contains(elem) // Uses the HashMap's contains() function
    }

    // Union operations
    ///////////////////

    /// Returns a iterator that yields references to all elements that 
    /// appear in both `self` and `other`.
    pub fn union<'a>(&'a self, other: &'a Self) -> Union<'a, T> {
        Union::build(&self.map, &other.map)
    }

    /// Consumes `other` to add all of its elements to `self`.
    pub fn extend(&mut self, other: Self) {
        let other_iter = other.map.into_iter();
        for key in other_iter {
            if !self.map.contains(&key.0) {
                self.put(key.0)
            }
        }
    } 

    //Intersection operations
    /////////////////////////

    /// Returns an iterator that yields references to elements contained in both sets.
    pub fn intersection<'a>(&'a self, other: &'a Self) -> Intersection<'a, T> {
        let (longer, shorter) = if self.map.len() >= other.map.len() {
            (&self.map, &other.map)
        } else {
            (&other.map, &self.map)
        };
        Intersection {
            lhs: longer,
            rhs_iter: shorter.keys(),
        }
    }

    /// Mutates `self` such that `self` only keeps elements that are also in `other`.
    /// Different from the function `retain` which takes a closure to only retain elements
    /// according to the predicate.
    ///
    /// NOTE: This function could also appropriately be called "intersect", but that was
    /// too close to the non-mutating version of this function called "intersection".
    // O(m*n)
    pub fn retain_all(&mut self, other: &Self) {
        // Collect keys to remove
        let keys_to_remove: Vec<_> = self.map.keys()
            .filter(|k| !other.map.contains(k))
            .cloned() // collect owned keys if necessary
            .collect();
    
        // Remove them safely
        for key in keys_to_remove {
            self.map.remove(&key);
        }
    }

    // Subtraction operations
    /////////////////////////

    /// Returns an iterator that yields references to elements in `self` but not `other`. 
    /// This operation is positionally dependent, and may yield different elements 
    /// depending on which set is set as `self` and which is set as `other`.
    pub fn difference<'a>(&'a self, other: &'a Self) -> Difference<'a, T> {
        //let (longer, shorter) = if self.map.len() <= other.map.len() {
        //    (&self.map, &other.map)
        //} else {
        //    (&other.map, &self.map)
        //};
        //Diff::new(longer, shorter)
        Difference {
            lhs_iter: self.map.keys(), 
            rhs: &other.map
        }

    }

    /// Returns an iterator that yields references to elements that appear in 
    /// either `self` or `other`, but not both. This operation yields 
    /// the same set regardless of which set is set as `self`.
    pub fn unique<'a>(&'a self, other: &'a Self) -> Unique<'a, T> {
        Unique::new(&self.map, &other.map)
    }

    // Filter operations
    ////////////////////

    // Mutates `S` to only contain values that adhere to the given predicate (lambda)
    //pub fn retain(&mut self, |predicate|) {}

}

pub struct Union<'a, K> {
    // Used "raw" for its "free" operations
    lhs: &'a probing_hash_table::HashMap<K, ()>,
    // Create explicit iterators outside of the next() implementation
    lhs_iter: probing_hash_table::Keys<'a, K, ()>, 
    rhs_iter: probing_hash_table::Keys<'a, K, ()>,
}
impl<'a, K> Union<'a, K> {
    // Constructor that takes map references to create iterators outside of the next()
    // implementation
    fn build(
        lhs: &'a probing_hash_table::HashMap<K, ()>, 
        rhs: &'a probing_hash_table::HashMap<K, ()>) 
    -> Union<'a, K> 
    where
        K: Debug + Eq + Hash + PartialEq,
    {
        Union {
            lhs_iter: lhs.keys(),
            rhs_iter: rhs.keys(),
            lhs,
        }
    }
}
impl<'a, K> Iterator for Union<'a, K>
where
    K: Debug + Eq + Hash,
{
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        // Yield elements from lhs iterator
        if let Some(key) = self.lhs_iter.next() {
            return Some(key);
        }

        // Then yield only unique elements from the rhs iterator
        self.rhs_iter.by_ref().find(|&key| !self.lhs.contains(key))
        //while let Some(key) = self.rhs_iter.next() {
        //    if !self.lhs.contains(key) {
        //        return Some(key);
        //    }
        //}
        //None
    }
}

pub struct Intersection<'a, K> {
    rhs_iter: probing_hash_table::Keys<'a, K, ()>,
    lhs: &'a probing_hash_table::HashMap<K, ()>, // needed to skip duplicates from rhs
}
impl<'a, K> Iterator for Intersection<'a, K>
where
    K: Debug + Eq + Hash,
{
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        // Yield only elements from both sets
        self.rhs_iter.by_ref().find(|&key| self.lhs.contains(key))
    }
}

pub struct Difference<'a, K> {
    lhs_iter: probing_hash_table::Keys<'a, K, ()>,
    rhs: &'a probing_hash_table::HashMap<K, ()>, // optional if needed
}
impl<'a, K> Iterator for Difference<'a, K>
where
    K: Debug + Eq + Hash,
{
    type Item = &'a K;

    // Yield only elements from self not contained in other
    fn next(&mut self) -> Option<Self::Item> {
        self.lhs_iter.find(|&k| !self.rhs.contains(k))
    }
}

pub struct Unique<'a, K> {
    lhs_iter: probing_hash_table::Keys<'a, K, ()>,
    rhs_iter: probing_hash_table::Keys<'a, K, ()>,
    lhs: &'a probing_hash_table::HashMap<K, ()>,
    rhs: &'a probing_hash_table::HashMap<K, ()>,
}
impl<'a, K> Unique<'a, K>
where
    K: Debug + Eq + Hash,
{
    pub fn new(lhs: &'a probing_hash_table::HashMap<K, ()>, rhs: &'a probing_hash_table::HashMap<K, ()>) -> Self {
        Self {
            lhs,
            rhs,
            lhs_iter: lhs.keys(),
            rhs_iter: rhs.keys(),
        }
    }
}
impl<'a, K> Iterator for Unique<'a, K>
where
    K: Debug + Eq + Hash,
{
    type Item = &'a K;

    // Yield only elements not contained across both sets
    fn next(&mut self) -> Option<Self::Item> {
        // First yield from rhs elements not in lhs
        if let Some(key) = self.rhs_iter.find(|&k| !self.lhs.contains(k)) {
            return Some(key);
        }

        // Then reverse the operation to yield lhs elements not in rhs
        self.lhs_iter.find(|&k| !self.rhs.contains(k))
    }
}


// Unit tests
/////////////

#[test]
// Basic function test
fn hash_set_test() {
    //Creates a new hash map
    let mut set = HashSet::<&str>::new();
    assert_eq!(set.size(), 0);
    assert_eq!(set.map.len(), 0);

    set.put("Peter");
    set.put("Brain");
    set.put("Bobson");
    set.put("Peter"); // Oopsiedoodle, a duplicate!
    set.put("Dichael");

    assert_eq!(set.size(), 4);
    assert_eq!(set.map.len(), 4);
    assert!(!set.is_empty());
    assert!(set.contains(&"Peter"));
    assert!(set.contains(&"Brain"));
    assert!(set.contains(&"Bobson"));
    assert!(set.contains(&"Dichael"));
}

#[test]
// Covers union() and extend()
fn union() {
    //Creates a new hash map
    let mut set1 = HashSet::<&str>::new();
    set1.put("Peter"); // +1
    set1.put("Brain"); // +1
    set1.put("Bobson"); // +1
    set1.put("Dichael"); // +1

    let mut set2 = HashSet::<&str>::new();
    set2.put("Remus"); // +1
    set2.put("Romulus"); // +1
    set2.put("Bobson"); 
    set2.put("Peter"); 

    let mut union: Vec<&str> = Vec::new();
    for e in set1.union(&set2) {
        union.push(*e); 
    }
    assert_eq!(union.len(), 6);
    eprintln!("{:#?}", union);
    // contains takes &T, which is &&str for the type
    assert!(union.contains(&"Peter"));
    assert!(union.contains(&"Brain"));
    assert!(union.contains(&"Bobson"));
    assert!(union.contains(&"Dichael"));

    // Consumes set2 and extends set1
    assert_eq!(set1.size(), 4);
    set1.extend(set2);
    assert_eq!(set1.size(), 6);
    assert!(set1.contains("Peter"));
    assert!(set1.contains("Brain"));
    assert!(set1.contains("Bobson"));
    assert!(set1.contains("Dichael"));
    assert!(set1.contains("Remus"));
    assert!(set1.contains("Romulus"));
    
    //panic!("MANUAL TEST FAILURE");
}

#[test]
// Covers intersection() and retain_all()
fn intersection() {
    //Creates a couple new hash sets
    let mut set1 = HashSet::<&str>::new();
    set1.put("Peter");
    set1.put("Brain");
    set1.put("Bobson");
    set1.put("Dichael");

    let mut set2 = HashSet::<&str>::new();
    set2.put("Remus");
    set2.put("Romulus");
    set2.put("Bobson"); 
    set2.put("Glank"); 
    set2.put("Flock"); 
    set2.put("Peter"); 

    // Creates a vec of borrowed keys contained only in both sets
    let mut intersection: Vec<&str> = Vec::new();
    for e in set1.intersection(&set2) {
        intersection.push(*e); 
    }
    eprintln!("{:#?}", intersection);
    assert_eq!(intersection.len(), 2);
    assert!(intersection.contains(&"Peter"));
    assert!(intersection.contains(&"Bobson"));

    // Mutates self to contain only keys ALSO contained in other
    eprintln!("{:#?}", set1.map);
    assert_eq!(set1.size(), 4);
    assert!(set1.contains(&"Peter"));
    assert!(set1.contains(&"Brain"));
    assert!(set1.contains(&"Bobson"));
    assert!(set1.contains(&"Dichael"));
    set1.retain_all(&set2); // Removes two entries
    eprintln!("{:#?}", set1.map);
    assert_eq!(set1.size(), 2);
    assert!(set1.contains(&"Peter"));
    assert!(set1.contains(&"Bobson"));

    //panic!("MANUAL TEST FAILURE");
}

#[test]
// Covers difference() and unique()
fn difference() {
    //Creates a couple new hash sets
    let mut set1 = HashSet::<&str>::new();
    set1.put("Peter"); 
    set1.put("Brain"); // +1
    set1.put("Bobson"); 
    set1.put("Dichael"); // +1 

    let mut set2 = HashSet::<&str>::new();
    set2.put("Remus"); // +1
    set2.put("Romulus"); // +1
    set2.put("Bobson"); 
    set2.put("Flock");  // +1
    set2.put("Peter"); 

    // Creates a vec of borrowed keys contained only in both sets
    let mut difference: Vec<&str> = Vec::new();
    for e in set1.difference(&set2) {
        difference.push(*e); 
    }
    eprintln!("{:#?}", difference);
    assert_eq!(difference.len(), 2);
    assert!(difference.contains(&"Brain"));
    assert!(difference.contains(&"Dichael"));

    // Proves that difference() is positionally dependent
    let mut difference: Vec<&str> = Vec::new();
    for e in set2.difference(&set1) {
        difference.push(*e); 
    }
    eprintln!("{:#?}", difference);
    assert_eq!(difference.len(), 3);
    assert!(difference.contains(&"Remus"));
    assert!(difference.contains(&"Romulus"));
    assert!(difference.contains(&"Flock"));

    // Creates a vec of borrowed keys contained only in both sets
    let mut unique: Vec<&str> = Vec::new();
    for e in set1.unique(&set2) {
        unique.push(*e); 
    }
    eprintln!("{:#?}", unique);
    assert_eq!(unique.len(), 5);
    assert!(unique.contains(&"Brain"));
    assert!(unique.contains(&"Dichael"));
    assert!(unique.contains(&"Remus"));
    assert!(unique.contains(&"Romulus"));
    assert!(unique.contains(&"Flock"));

    //panic!("MANUAL TEST FAILURE");
}
