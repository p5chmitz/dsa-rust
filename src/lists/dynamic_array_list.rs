/////////////////////////////////
/** A dynamic array-based list */
/////////////////////////////////

use std::cell::RefCell;

#[derive(Debug)] // Required for generic array initialization
pub struct List<T> {
    data: Vec<Option<RefCell<T>>>,
    size: usize
}
impl<T: Clone> Clone for List<T> {
    fn clone(&self) -> List<T> {
        List {
            data: self.data.clone(),
            size: self.size,
        }
    }
}
/** The List's public API contains the following functions: 
 * - new() -> List<T>
 * - is_empty(self) -> bool
 * - get(&self, i: usize) -> Option<&T>
 * - set(i, e):
 * - remove(&mut self, i: usize) -> Option<T>
* NOTE: Rust only allows arrays to be instantiated with constants, which are immutable. Even the
* Vec type in the standard library uses an internal module called RawVec to circumvent this
* constraint. In order to avoid reimplementing that module, this module uses Vec as its base heap
* storage vehicle. Please just pretend that the underlying allocations are static.
* */
impl<T: Clone> List<T> {
    /** Creates a new generic list */
    pub fn new() -> List<T> {
        List {
            data: Vec::new(),
            size: 0
        }
    }
    pub fn is_empty(self) -> bool {
        self.size == 0
    }
    /** Adds a new element e to the list at index i, performs bounds checks and if i is larger than
     * size, the list resizes to be 2 * i*/
    pub fn set(&mut self, e: T, i: usize) {
        // Resize empty lists to take the first element
        if self.size <= i {
            let new_size: usize = (i + 1).max(self.data.len() * 2);
            self.data.resize(new_size, None);
        }
        // Add element to the list and increment the list size
        self.data[i] = Some(e.into());
        self.size += 1
    }
    /** Gets (but does not remove) data at index i */
    pub fn get(&self, i: usize) -> Option<T> {
        self.data[i].as_ref().map(|ref_cell| ref_cell.borrow().clone())
    }
    /** Removes and returns the data at index i */
    pub fn remove(&mut self, i: usize) -> Option<T> {
        // Attempt to take the value out of the RefCell and decrement the list
        if let Some(ref_cell) = self.data[i].take() {
            self.size -= 1;
            return Some(ref_cell.into_inner());
        }
        None
    }
    /** Clears all elements from the list and resizes to 1 */
    pub fn clear(&mut self) {
        for i in (0..self.data.len()).rev() {
            self.data.remove(i);
        }
        self.data.resize(0, None);
        self.size = 0;
    }
    /** Removes all excess elements from the list */
    // TODO: Implement
    pub fn trim(&mut self) {}
}

/** This driver illustrates an array implementation for the list ADT... using no vectors */
#[test]
pub fn example() {
    // Creates new list
    let mut list: List<String> = List::new();

    // Sets index 0 to a first name, tests the get method
    let mut name = "Chester".to_string();
    list.set(name, 0);
    let mut reference: String = list.get(0).unwrap();
    assert_eq!(reference, "Chester".to_string());

    // Tests that the size is appropriate after the first add
    assert_eq!(list.size, 1);

    // Sets index 1 to last name, tests the get method
    name = "Copperpot".to_string();
    list.set(name, 1);
    reference = list.get(1).unwrap();
    assert_eq!(reference, "Copperpot".to_string()); 

    // Tests that the size is appropriate 
    assert_eq!(list.size, 2);
    assert_eq!(list.data.len(), 2);

    // Tests that indexes 0 and 1 are different
    let left: String = list.get(0).unwrap();
    let right: String = list.get(1).unwrap();
    assert_ne!(left, right);

    // Tests the resize logic
    name = "Oregon".to_string();
    list.set(name, 2);
    assert_eq!(list.size, 3);
    assert_eq!(list.data.len(), 4);

    name = "USA".to_string();
    list.set(name, 3);
    assert_eq!(list.size, 4);
    assert_eq!(list.data.len(), 8);

    name = "Wild".to_string();
    list.set(name, 21);
    assert_eq!(list.size, 5);
    assert_eq!(list.data.len(), 22);

    name = "Country".to_string();
    list.set(name, 22);
    assert_eq!(list.size, 6);
    assert_eq!(list.data.len(), 44);

    // The list is now empty
    list.clear();
    assert_eq!(list.size, 0);
    assert_eq!(list.data.len(), 0);
}

/** Just used to debug print the list to visualize how the list works */
pub fn visualize() {
    // Creates new list
    let mut list: List<String> = List::new();

    // Sets index 0 to a first name, tests the get method
    let mut name = "Chester".to_string();
    list.set(name, 0);
    println!("Generic list: \n{:?}", list); // Capacity == 1

    name = "Copperpot".to_string();
    list.set(name, 1);

    name = "Oregon".to_string();
    list.set(name, 2);

    name = "USA".to_string();
    list.set(name, 3);
    println!("Generic list: \n{:?}", list); // Capacity == 8

    name = "Wild".to_string();
    list.set(name, 21);
    println!("Generic list: \n{:?}", list); // Capacity == 22

    name = "Country".to_string();
    list.set(name, 22);
    println!("Generic list: \n{:?}", list); // Capacity == 44

    list.clear();
    println!("Generic list: \n{:?}", list); // Capacity == 0

}
