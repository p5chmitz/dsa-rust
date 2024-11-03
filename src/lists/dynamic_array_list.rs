/////////////////////////////////
/** A dynamic array-based list */
/////////////////////////////////
use std::cell::RefCell;

#[derive(Debug)] // Required for generic array initialization
/** The List's public API contains the following functions:
 - new() -> List<T>
 - is_empty(self) -> bool
 - add(&mut self, e: T, i: usize) -> Result<(), &'static str>
 - get(&self, i: usize) -> Option<T>
 - remove(&mut self, i: usize) -> Option<T>
* NOTE: Rust only allows arrays to be instantiated with constants, which are immutable. Even the
* Vec type in the standard library uses an internal module called RawVec to circumvent this
* constraint. In order to avoid reimplementing that module, this module uses Vec as its base heap
* storage vehicle. Please just pretend that the underlying allocations are static.
* */
pub struct List<T> {
    data: Vec<Option<RefCell<T>>>,
    size: usize,
}
impl<T: Clone> Clone for List<T> {
    fn clone(&self) -> List<T> {
        List {
            data: self.data.clone(),
            size: self.size,
        }
    }
}
impl<T: Clone> List<T> {
    /** Creates a new generic list */
    pub fn new() -> List<T> {
        List {
            data: vec![None], // Creates a Vec and sets the only index to None
            size: 0,
        }
    }
    pub fn is_empty(self) -> bool {
        self.size == 0
    }
    /** Attempts to add a new element e to the list at index i;
     * Checks that i is within the list's capacity (self.data.len()), otherwise returns an error;
     * If i is equal to the list's capacity the list resizes to be 2x capacity */
    pub fn add(&mut self, e: T, i: usize) -> Result<(), &'static str> {
        // Returns error if larger than the current capacity
        if i > self.data.len() {
            return Err("Error: Index out of bounds");
        // Resizes the list if i == capacity
        } else if i == self.data.len() {
            // Resizes the Vec to be 2x capacity and initializes empty elements as None
            self.data.resize(2 * self.data.len(), None);
        }
        // Add element to the list and increment the list size
        self.data[i] = Some(e.into());
        self.size += 1;
        Ok(())
    }
    /** Gets (but does not remove) data at index i */
    pub fn get(&self, i: usize) -> Option<T> {
        // A naive approach to extraction
        //let some_cell_ref: Option<&RefCell<T>> = self.data[i].as_ref();
        //let cell_ref: &RefCell<T> = some_cell_ref.unwrap();
        //let some_val: Option<T> = Some(cell_ref.borrow().clone());
        //some_val
        // Using map removes the need to declare/initialize intermediate variables
        self.data[i]
            .as_ref()
            .map(|ref_cell| ref_cell.borrow().clone())
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
    /** Resizes the capacity to match the number of elements */
    pub fn trim(&mut self) {
        self.data.resize(self.size, None);
    }
}

/** This driver illustrates an array implementation for the list ADT... using no vectors */
#[test]
pub fn example() {
    // Creates new list
    let mut list: List<String> = List::new();

    // Sets index 0 to a first name, tests the get method
    let mut name = "Chester".to_string();
    list.add(name, 0).unwrap();
    let mut reference: String = list.get(0).unwrap();
    assert_eq!(reference, "Chester".to_string());

    // Tests that the size is appropriate after the first add
    assert_eq!(list.size, 1);

    // Sets index 1 to last name, tests the get method
    name = "Copperpot".to_string();
    list.add(name, 1).unwrap();
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
    list.add(name, 2).unwrap();
    assert_eq!(list.size, 3);
    assert_eq!(list.data.len(), 4);

    name = "USA".to_string();
    list.add(name, 3).unwrap();
    assert_eq!(list.size, 4);
    assert_eq!(list.data.len(), 4);

    // Attempts to add an entry to some OOB index
    name = "Wild".to_string();
    let _ = list.add(name, 21).is_err();
    assert_eq!(list.size, 4);
    assert_eq!(list.data.len(), 4);

    // Adds an entry, doubling the capacity
    name = "Country".to_string();
    list.add(name, 4).unwrap();
    assert_eq!(list.size, 5);
    assert_eq!(list.data.len(), 8);

    // Removes an element which doens't affect capacity
    list.remove(4).unwrap();
    assert_eq!(list.size, 4);
    assert_eq!(list.data.len(), 8);

    // Trims the capacity to the current size
    list.trim();
    assert_eq!(list.size, 4);
    assert_eq!(list.data.len(), 4);

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
    list.add(name, 0).unwrap();
    println!("Generic list: \n{:?}", list); // Capacity == 1

    name = "Copperpot".to_string();
    list.add(name, 1).unwrap();

    name = "Oregon".to_string();
    list.add(name, 2).unwrap();

    name = "USA".to_string();
    list.add(name, 3).unwrap();
    println!("Generic list: \n{:?}", list); // Capacity == 8

    name = "Wild".to_string();
    list.add(name, 21).unwrap();
    println!("Generic list: \n{:?}", list); // Capacity == 22

    name = "Country".to_string();
    list.add(name, 22).unwrap();
    println!("Generic list: \n{:?}", list); // Capacity == 44

    list.clear();
    println!("Generic list: \n{:?}", list); // Capacity == 0
}
