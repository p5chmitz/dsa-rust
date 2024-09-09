#![allow(dead_code)]

// Singly linked list with single ownership as a stack implementation
// This version is adapted from the book Entirely Too Many Linked Lists
pub mod owned_singly_linked_list {

    type Link<T> = Option<Box<Node<T>>>; // Brevity is the soul of wit

    struct Node<T> {
        elem: T,
        next: Link<T>,
    }

    // Basic list structure
    pub struct List<T> {
        head: Link<T>,
    }
    impl<T> List<T> {
        // List API
        ///////////

        /** Creates a new list */
        pub fn new() -> Self {
            List { head: None }
        }
        /** Adds a new list node */
        pub fn push(&mut self, elem: T) {
            let new_node = Box::new(Node {
                elem,
                next: self.head.take(),
            });
            self.head = Some(new_node);
        }
        /** Returns (and removes) the element from the top of the list */
        pub fn pop(&mut self) -> Option<T> {
            self.head.take().map(|node| {
                self.head = node.next;
                node.elem
            })
        }
        /** Returns (and removes) the element at the ith index */
        pub fn remove(&mut self, index: u32) -> Option<T> {
            self.head.take().map(|node| {
                self.head = node.next;
                node.elem
            })
        }
        /** Returns (without removing) the element from the top of the list */
        pub fn peek(&self) -> Option<&T> {
            self.head.as_ref().map(|node| &node.elem)
        }

        // Utility functions
        ////////////////////

        // Iterator functions convert instances into iterators
        pub fn iter(&self) -> Iter<'_, T> {
            Iter {
                next: self.head.as_deref(),
            }
        }
        pub fn into_iter(self) -> IntoIter<T> {
            IntoIter(self)
        }
        pub fn iter_mut(&mut self) -> IterMut<'_, T> {
            IterMut {
                next: self.head.as_deref_mut(),
            }
        }
    }

    // Trait implementations
    ////////////////////////

    // Customizes how elements are dropped, providing structural integrity
    // when deleting a node of the list to avoid overflowing the stack
    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            let mut cur_link = self.head.take();
            while let Some(mut boxed_node) = cur_link {
                cur_link = boxed_node.next.take();
            }
        }
    }

    // Implements the Iterator trait, provides required types/trait definitions for
    // associated function implementations which yield immutable references (iter),
    // owned values (into_iter), and mutable references (iter_mut)
    pub struct Iter<'a, T> {
        next: Option<&'a Node<T>>,
    }
    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            self.next.map(|node| {
                self.next = node.next.as_deref();
                &node.elem
            })
        }
    }

    pub struct IntoIter<T>(List<T>);
    impl<T> Iterator for IntoIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            // access fields of a tuple struct numerically
            self.0.pop()
        }
    }

    pub struct IterMut<'a, T> {
        next: Option<&'a mut Node<T>>,
    }
    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;
        fn next(&mut self) -> Option<Self::Item> {
            self.next.take().map(|node| {
                self.next = node.next.as_deref_mut();
                &mut node.elem
            })
        }
    }
}

pub fn list_adt_driver_0() {
    use owned_singly_linked_list::List;
    let mut list: List<String> = List::new();
    list.push("Peter".to_string());
    list.push("Brain".to_string());
    list.push("Bobson".to_string());
    list.push("Dingus".to_string());
    for e in list.iter() {
        let node = e.to_string();
        println!("{}", node)
    }
    println!("")
}

/** Adapted list ADT for the podium example */
pub mod podium_singly_linked_list {
    // Its come to using raw pointers
    use std::ptr::NonNull;

    //type Link = Option<Box<Node>>;
    type Link = Option<NonNull<Node>>;

    pub struct Node {
        pub name: String,
        pub score: i32,
        next: Link,
    }

    // Basic list structure
    pub struct List {
        head: Link,
        length: u32,
    }
    impl<'a> List {
        // List API
        ///////////

        /** Creates a new list */
        pub fn new() -> Self {
            List {
                head: None,
                length: 0,
            }
        }
        // My original singly-linked adaptation
        /** Inserts node into middle of sorted list */
        //pub fn insert(&mut self, name: String, score: i32) {
        //    // Creates a new node to insert
        //    let mut new_node = Box::new(Node {
        //        name,
        //        score,
        //        next: None,
        //    });
        //    // Special case for inserting at the head
        //    if self.head.is_none() || self.head.as_ref().unwrap().score < score {
        //        new_node.next = self.head.take();
        //        self.head = Some(new_node);
        //        return;
        //    }
        //    // Find the correct position for the new node
        //    let mut current = &mut self.head;
        //    while let Some(ref mut node) = *current {
        //        if node.next.is_none() || node.next.as_ref().unwrap().score < score {
        //            // Insert before node.next
        //            new_node.next = node.next.take();
        //            node.next = Some(new_node);
        //            return;
        //        }
        //        current = &mut node.next;
        //    }
        //    // If the score is low enough insert it at the end
        //    *current = Some(new_node);
        //}
        pub fn insert(&mut self, name: String, score: i32) {
            // Creates a new node to insert
            let new_node = Box::new(Node {
                name: name.clone(),
                score,
                next: None,
            });
            // Special case for inserting at the head
            let new_node_ptr = Box::into_raw(new_node);
            let new_node_nonnull = NonNull::new(new_node_ptr).unwrap();
            if self.head.is_none() || unsafe { (*self.head.unwrap().as_ptr()).score < score } {
                unsafe {
                    // Set the new node as the head
                    let head = self.head.take();
                    let mut new_node = Box::from_raw(new_node_ptr);
                    new_node.next = head;
                    self.head = Some(new_node_nonnull);
                }
                self.length += 1;
                return;
            }
            // Find the correct position for the new node
            let mut current = self.head;
            while let Some(current_ptr) = current {
                unsafe {
                    // Returns a shared reference
                    let current_ref = current_ptr.as_ref();
                    if current_ref.name == name.clone() {
                        // Returns a NonNull *mut T raw pointer
                        let next_node = (*current_ptr.as_ptr()).next.take();

                        // If removing the head
                        if self.head == Some(current_ptr.clone()) {
                            self.head = next_node;
                        } else {
                            let mut prev_ptr: Option<NonNull<Node>> = self.head;
                            while let Some(prev) = prev_ptr {
                                let prev_ref: NonNull<Node> = prev;
                                // Dereference the raw pointer to access the member fields
                                if (*prev_ref.as_ptr()).next == Some(current_ptr.clone()) {
                                    (*prev_ref.as_ptr()).next = next_node;
                                    break;
                                }
                                prev_ptr = (*prev_ref.as_ptr()).next;
                            }
                        }

                        self.length = self.length.checked_sub(1).unwrap_or(0);
                        
                        // Convert current_ptr to a Box and then back to a raw pointer to prevent memory leaks
                        let _ = Box::from_raw(current_ptr.as_ptr());

                        //return Some(current_ptr.clone());
                    }
                    current = (*current_ptr.as_ptr()).next;
                }
            }
            //while let Some(ref mut node) = *current {
            //    current = Box::from_raw(current_ptr.as_ptr());
            //    if unsafe { (*current.unwrap().as_ptr()).next.is_none() }
            //        || unsafe { (*current.unwrap().as_ptr()).score < score }
            //    {
            //        // Insert before node.next
            //        new_node.next = unsafe { (*current.unwrap().as_ptr()).next.take() };
            //        unsafe { (*current.unwrap().as_ptr()).next.take() = *new_node };
            //        return;
            //    }
            //    current = &mut node.next;
            //}
            // If the score is low enough insert it at the end
            //*current = Some(new_node);
        }
        /** Removes a named node */
        // TODO: get this to work
        //pub fn remove(&mut self, name: String) {
        //    // Assigns current as the current (starting) Link
        //    let mut iter_node = self.head;
        //    // Handles edge case where removal is the head node
        //    unsafe {
        //    if (*iter_node.expect("Couldn't get the name from the raw pointer").as_ptr()).name == name {
        //        println!("{name} is at the front of the list");
        //        return;
        //    } else {
        //        // Handles all other node matches
        //        while let Some(ref mut node) = *iter_node {
        //            let _ = match node.next.as_ref() {
        //                Some(_) => {
        //                    println!("{name} is definitely somewhere in the list!");
        //                    return;
        //                }
        //                None => {
        //                    let msg = format!("List doesn't contain {name}");
        //                    println!("{msg}");
        //                    return;
        //                }
        //            };
        //        }
        //    }}
        //}

        //// Combines the book's three delete functions within the doubly-linked implementation
        //pub fn yank(&mut self, index: u32) -> Option<Node> {
        //    // Check for OOB error
        //    if self.length < index {
        //        panic!("Index out of bounds");
        //    }

        //    // Delete the head node
        //    if index == 0 || self.head.is_none() {
        //        // Safety: head_ptr points to a leaked boxed node managed by this list
        //        // We reassign pointers that pointed to the head node
        //        if self.length == 0 {
        //            return None;
        //        }

        //        self.head.map(|head_ptr| unsafe {
        //            let old_head = Box::from_raw(head_ptr.as_ptr());
        //            match old_head.next {
        //                Some(mut next_ptr) => (*next_ptr.as_mut()).prev = None,
        //                None => None,
        //            }
        //            self.head = old_head.next;
        //            self.length = self.length.checked_add_signed(-1).unwrap_or(0);
        //            old_head.data
        //        })
        //        // None
        //    }

        //    // Delete last node
        //    if self.length == index {
        //        // Safety: tail_ptr points to a leaked boxed node managed by this list
        //        // We reassign pointers that pointed to the tail node
        //        self.tail.map(|tail_ptr| unsafe {
        //            let old_tail = Box::from_raw(tail_ptr.as_ptr());
        //            match old_tail.prev {
        //                Some(mut prev) => prev.as_mut().next = None,
        //                None => self.head = None,
        //            }
        //            self.tail = old_tail.prev;
        //            self.length -= 1;
        //            old_tail.data
        //        })
        //    }

        //    if let Some(mut ith_node) = self.head {
        //        for _ in 0..index {
        //            unsafe {
        //                match (*ith_node.as_ptr()).next {
        //                    None => panic!("Index out of bounds"),
        //                    Some(next_ptr) => ith_node = next_ptr,
        //                }
        //            }
        //        }

        //        unsafe {
        //            let old_ith = Box::from_raw(ith_node.as_ptr());
        //            if let Some(mut prev) = old_ith.prev {
        //                prev.as_mut().next = old_ith.next;
        //            }
        //            if let Some(mut next) = old_ith.next {
        //                next.as_mut().prev = old_ith.prev;
        //            }

        //            self.length -= 1;
        //            Some(old_ith.data)
        //        }
        //    } else {
        //        None
        //    }
        //}

        //        pub fn delete_head(&mut self) -> Option<T> {
        //            // Safety: head_ptr points to a leaked boxed node managed by this list
        //            // We reassign pointers that pointed to the head node
        //            if self.length == 0 {
        //                return None;
        //            }
        //
        //            self.head.map(|head_ptr| unsafe {
        //                let old_head = Box::from_raw(head_ptr.as_ptr());
        //                match old_head.next {
        //                    Some(mut next_ptr) => next_ptr.as_mut().prev = None,
        //                    None => self.tail = None,
        //                }
        //                self.head = old_head.next;
        //                self.length = self.length.checked_add_signed(-1).unwrap_or(0);
        //                old_head.data
        //            })
        //            // None
        //        }
        //
        //        pub fn delete_tail(&mut self) -> Option<T> {
        //            // Safety: tail_ptr points to a leaked boxed node managed by this list
        //            // We reassign pointers that pointed to the tail node
        //            self.tail.map(|tail_ptr| unsafe {
        //                let old_tail = Box::from_raw(tail_ptr.as_ptr());
        //                match old_tail.prev {
        //                    Some(mut prev) => prev.as_mut().next = None,
        //                    None => self.head = None,
        //                }
        //                self.tail = old_tail.prev;
        //                self.length -= 1;
        //                old_tail.data
        //            })
        //        }
        //
        //        pub fn delete(&mut self, index: u32) -> Option<T> {
        //            if self.length < index {
        //                panic!("Index out of bounds");
        //            }
        //
        //            if index == 0 || self.head.is_none() {
        //                return self.delete_head();
        //            }
        //
        //            if self.length == index {
        //                return self.delete_tail();
        //            }
        //
        //            if let Some(mut ith_node) = self.head {
        //                for _ in 0..index {
        //                    unsafe {
        //                        match (*ith_node.as_ptr()).next {
        //                            None => panic!("Index out of bounds"),
        //                            Some(next_ptr) => ith_node = next_ptr,
        //                        }
        //                    }
        //                }
        //
        //                unsafe {
        //                    let old_ith = Box::from_raw(ith_node.as_ptr());
        //                    if let Some(mut prev) = old_ith.prev {
        //                        prev.as_mut().next = old_ith.next;
        //                    }
        //                    if let Some(mut next) = old_ith.next {
        //                        next.as_mut().prev = old_ith.prev;
        //                    }
        //
        //                    self.length -= 1;
        //                    Some(old_ith.data)
        //                }
        //            } else {
        //                None
        //            }
        //        }

        // Utility functions
        ////////////////////

        // Iterator functions convert instances into iterators
        pub fn iter(&self) -> Iter<'_> {
            Iter {
                next: self.head.as_ref(),
            }
        }
        pub fn into_iter(self) -> IntoIter {
            IntoIter { next: self.head }
        }
    }

    // Trait implementations
    ////////////////////////

    // Referenced iterator
    pub struct Iter<'a> {
        next: Option<&'a NonNull<Node>>,
    }

    impl<'a> Iterator for Iter<'a> {
        type Item = &'a NonNull<Node>;
        fn next(&mut self) -> Option<Self::Item> {
            unsafe {
                self.next.map(|node| {
                self.next = (*node.as_ptr()).next.as_ref();
                node
            })}
        }
    }
    // Owned iterator
    pub struct IntoIter {
        next: Link,
    }
    impl Iterator for IntoIter {
        type Item = NonNull<Node>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next.take() {
                Some(node) => {
                    unsafe { 
                        self.next = (*node.as_ptr()).next.take(); 
                        Some(node)
                    }
                }
                _ => None,
            }
        }
    }
}

//#[test]
//pub fn linked_list_test() {
//    use podium_singly_linked_list::List;
//    let mut podium = List::new();
//    podium.insert("Peter".to_string(), 1223);
//    podium.insert("Brain".to_string(), 616);
//    podium.insert("Dingus".to_string(), 420);
//    podium.insert("Dangus".to_string(), 1423);
//    podium.insert("Bobson".to_string(), 3434);
//    podium.insert("Remus".to_string(), 9838);
//    podium.insert("Romulus".to_string(), 18423);
//    let mut v = vec![];
//    for (i, e) in podium.into_iter().enumerate() {
//        v.push(e.name);
//        if i == 2 {
//            break;
//        }
//    }
//    assert_eq!(
//        v,
//        vec![
//            "Romulus".to_string(),
//            "Remus".to_string(),
//            "Bobson".to_string()
//        ]
//    )
//}

pub fn list_adt_driver_1() {
    use podium_singly_linked_list::List;
    let mut podium = List::new();
    podium.insert("Peter".to_string(), 1223);
    podium.insert("Brain".to_string(), 616);
    podium.insert("Dingus".to_string(), 420);
    podium.insert("Romulus".to_string(), 18423);
    podium.insert("Dangus".to_string(), 1423);
    podium.insert("Bobson".to_string(), 3434);
    podium.insert("Remus".to_string(), 9838);

    println!("Initial linked list:");
    for (i, e) in podium.iter().enumerate() {
        let name = unsafe { e.as_ref().name.clone() };
        let score = unsafe { e.as_ref().score };
        println!("{:>2}: {:<8} {:>6}", i + 1, name, score);
    }
    //podium.remove("Romulus".to_string());
    //podium.remove("Dingus".to_string());
    //podium.remove("Steven".to_string());

    println!("Final (linked list) podium:");
    for (i, e) in podium.into_iter().enumerate() {
        unsafe { println!("{:>2}: {:<8} {:>6}", i + 1, e.as_ref().name, e.as_ref().score); }
        if i == 2 {
            break;
        }
    }
}

/**
 doubly-linked structure via
https://github.com/TheAlgorithms/Rust/blob/master/src/data_structures/linked_list.rs

Basic list type instances contain a length, two node references (to head and tail nodes),
and a zero-sized type that tells the compiler that your type acts as though
it stores a value of type `T`, even though it doesn't:
pub LinkedList {
    pub length: u32,
    pub head: Option<NonNull<Node<T>>>,
    pub tail: Option<NonNull<Node<T>>>,
    marker: PhantomData<Box<Node<T>>>,
}

Each element consists of the following information:
pub Node {
    pub data: Data {
        pub name: String,
        pub score: i32
    },
    pub next: Option<NonNull<Node<T>>>
    prev: Option<NonNull<Node<T>>>
}
*/
pub mod podium_doubly_linked_list {

    use std::fmt::{self, Display, Formatter};
    use std::marker::PhantomData;
    use std::ptr::NonNull;

    // The meat of the node
    pub struct Data {
        pub name: String,
        pub score: i32,
    }
    impl Data {
        fn build(name: String, score: i32) -> Self {
            Self { name, score }
        }
    }

    // Node that holds a generic data instance and some pointers
    pub struct Node<T> {
        pub data: T,
        pub next: Option<NonNull<Node<T>>>,
        prev: Option<NonNull<Node<T>>>,
    }
    impl<T> Node<T> {
        fn new(t: T) -> Node<T> {
            Node {
                data: t,
                prev: None,
                next: None,
            }
        }
    }

    pub struct LinkedList<T> {
        pub length: u32,
        pub head: Option<NonNull<Node<T>>>,
        pub tail: Option<NonNull<Node<T>>>,
        // Act like we own boxed nodes since we construct and leak them
        marker: PhantomData<Box<Node<T>>>,
    }

    impl<T> Default for LinkedList<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> LinkedList<T> {
        // Creates a new list OR a new node
        pub fn new() -> Self {
            Self {
                length: 0,
                head: None,
                tail: None,
                marker: PhantomData,
            }
        }

        pub fn insert_at_head(&mut self, obj: T) {
            let mut node = Box::new(Node::new(obj));
            node.next = self.head;
            node.prev = None;
            let node_ptr = NonNull::new(Box::into_raw(node));
            match self.head {
                None => self.tail = node_ptr,
                Some(head_ptr) => unsafe { (*head_ptr.as_ptr()).prev = node_ptr },
            }
            self.head = node_ptr;
            self.length += 1;
        }

        pub fn insert_at_tail(&mut self, obj: T) {
            let mut node = Box::new(Node::new(obj));
            node.next = None;
            node.prev = self.tail;
            let node_ptr = NonNull::new(Box::into_raw(node));
            match self.tail {
                None => self.head = node_ptr,
                Some(tail_ptr) => unsafe { (*tail_ptr.as_ptr()).next = node_ptr },
            }
            self.tail = node_ptr;
            self.length += 1;
        }

        pub fn insert_at_ith(&mut self, index: u32, obj: T) {
            // Checks for edge cases including out-of-bounds, head, and tail insertions
            if self.length < index {
                panic!("Index out of bounds");
            }
            if index == 0 || self.head.is_none() {
                self.insert_at_head(obj);
                return;
            }
            if self.length == index {
                self.insert_at_tail(obj);
                return;
            }

            if let Some(mut ith_node) = self.head {
                // Traverses the list until it gets to the ith node
                for _ in 0..index {
                    unsafe {
                        match (*ith_node.as_ptr()).next {
                            None => panic!("Index out of bounds"),
                            Some(next_ptr) => ith_node = next_ptr,
                        }
                    }
                }
                // Creates a new node with the obj argument,
                // resets all the pointers around the node
                let mut node = Box::new(Node::new(obj));
                unsafe {
                    node.prev = (*ith_node.as_ptr()).prev;
                    node.next = Some(ith_node);
                    if let Some(p) = (*ith_node.as_ptr()).prev {
                        let node_ptr = NonNull::new(Box::into_raw(node));
                        println!("{:?}", (*p.as_ptr()).next);
                        (*p.as_ptr()).next = node_ptr;
                        (*ith_node.as_ptr()).prev = node_ptr;
                        self.length += 1;
                    }
                }
            }
        }
        /** Inserts node into middle of sorted list */
        // Reminder: Self is LinkedList<T>
        //pub fn sorted_insert(&mut self, name: String, score: i32) {
        //    // Creates a new data/node instance from the function args
        //    let new_node = Node::new(Data::build(name, score));

        //    // Special case for inserting at the head
        //    if self.head.is_none() || self.head.as_ref().unwrap().score < score {
        //        new_node.next = self.head.take();
        //        self.head = Some(new_node);
        //        return;
        //    }
        //    // Find the correct position for the new node
        //    let mut current = &mut self.head;
        //    while let Some(ref mut node) = *current {
        //        if node.next.is_none() || node.next.as_ref().unwrap().score < score {
        //            // Insert before node.next
        //            new_node.next = node.next.take();
        //            node.next = Some(new_node);
        //            return;
        //        }
        //        current = &mut node.next;
        //    }
        //    // If the score is low enough insert it at the end
        //    *current = Some(new_node);
        //}

        pub fn delete_head(&mut self) -> Option<T> {
            // Safety: head_ptr points to a leaked boxed node managed by this list
            // We reassign pointers that pointed to the head node
            if self.length == 0 {
                return None;
            }

            self.head.map(|head_ptr| unsafe {
                let old_head = Box::from_raw(head_ptr.as_ptr());
                match old_head.next {
                    Some(mut next_ptr) => next_ptr.as_mut().prev = None,
                    None => self.tail = None,
                }
                self.head = old_head.next;
                self.length = self.length.checked_add_signed(-1).unwrap_or(0);
                old_head.data
            })
            // None
        }

        pub fn delete_tail(&mut self) -> Option<T> {
            // Safety: tail_ptr points to a leaked boxed node managed by this list
            // We reassign pointers that pointed to the tail node
            self.tail.map(|tail_ptr| unsafe {
                let old_tail = Box::from_raw(tail_ptr.as_ptr());
                match old_tail.prev {
                    Some(mut prev) => prev.as_mut().next = None,
                    None => self.head = None,
                }
                self.tail = old_tail.prev;
                self.length -= 1;
                old_tail.data
            })
        }

        pub fn delete_ith(&mut self, index: u32) -> Option<T> {
            if self.length < index {
                panic!("Index out of bounds");
            }

            if index == 0 || self.head.is_none() {
                return self.delete_head();
            }

            if self.length == index {
                return self.delete_tail();
            }

            if let Some(mut ith_node) = self.head {
                for _ in 0..index {
                    unsafe {
                        match (*ith_node.as_ptr()).next {
                            None => panic!("Index out of bounds"),
                            Some(next_ptr) => ith_node = next_ptr,
                        }
                    }
                }

                unsafe {
                    let old_ith = Box::from_raw(ith_node.as_ptr());
                    if let Some(mut prev) = old_ith.prev {
                        prev.as_mut().next = old_ith.next;
                    }
                    if let Some(mut next) = old_ith.next {
                        next.as_mut().prev = old_ith.prev;
                    }

                    self.length -= 1;
                    Some(old_ith.data)
                }
            } else {
                None
            }
        }

        pub fn get(&self, index: i32) -> Option<&T> {
            Self::get_ith_node(self.head, index).map(|ptr| unsafe { &(*ptr.as_ptr()).data })
        }

        fn get_ith_node(node: Option<NonNull<Node<T>>>, index: i32) -> Option<NonNull<Node<T>>> {
            match node {
                None => None,
                Some(next_ptr) => match index {
                    0 => Some(next_ptr),
                    _ => Self::get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
                },
            }
        }
    }

    impl<T> Drop for LinkedList<T> {
        fn drop(&mut self) {
            // Pop items until there are none left
            while self.delete_head().is_some() {}
        }
    }

    impl<T> Display for LinkedList<T>
    where
        T: Display,
    {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            match self.head {
                Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
                None => Ok(()),
            }
        }
    }

    impl<T> Display for Node<T>
    where
        T: Display,
    {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            match self.next {
                Some(node) => write!(f, "{}, {}", self.data, unsafe { node.as_ref() }),
                None => write!(f, "{}", self.data),
            }
        }
    }
}

/** Illustrates a doubly-linked list */
pub fn list_adt_driver_2() {
    use podium_doubly_linked_list::{Data, LinkedList, Node};
    let mut _podium: LinkedList<Node<Data>> = LinkedList::new();
    //podium.insert("Peter".to_string(), 1223);
    //podium.insert("Brain".to_string(), 616);
    //podium.insert("Dingus".to_string(), 420);
    //podium.insert("Dangus".to_string(), 1423);
    //podium.insert("Bobson".to_string(), 3434);
    //podium.insert("Remus".to_string(), 9838);
    //podium.insert("Romulus".to_string(), 18423);

    //println!("Initial linked list:");
    //for (i, e) in podium.iter().enumerate() {
    //    println!("{:>2}: {:<8} {:>6}", i + 1, e.name, e.score);
    //}

    //podium.remove("Romulus".to_string());
    //podium.remove("Dingus".to_string());
    //podium.remove("Steven".to_string());

    //println!("Final (linked list) podium:");
    //for (i, e) in podium.into_iter().enumerate() {
    //    println!("{:>2}: {:<8} {:>6}", i + 1, e.name, e.score);
    //    if i == 2 {
    //        break;
    //    }
    //}
}

pub mod singly_linked_list {

    pub struct Node {
        name: String,
        score: i32,
        next: Option<Box<Node>>
    }
    impl Node {
        // Creates a new node
        pub fn new(name: String, score: i32) -> Node {
            Node {
                name,
                score,
                next: None
            }
        }
    }
    pub struct List {
        head: Option<Box<Node>>, // Adding an extra box just in case things get wild
        length: usize,
    }
    impl List {
        // Creates a new list
        pub fn new() -> List {
            List {
                head: None,
                length: 0
            }
        }
        // Inserts a node, sorted by its score
        pub fn insert(&mut self, node: Node) {
            // Handle the special case of inserting at the head
            if self.head.is_none() || self.head.as_ref().unwrap().score <= node.score {
                let mut new_head = Box::new(node);
                println!("A new star emerges");
                new_head.next = self.head.take();
                self.head = Some(new_head);
                self.length += 1;
                return
            }

            // Traverse the list to find the insertion point
            let mut iter_node = &mut self.head;
            while let Some(ref mut peek) = iter_node {
                if peek.next.is_none() || peek.next.as_ref().unwrap().score <= node.score {
                    let mut new_node = Box::new(node);
                    new_node.next = peek.next.take();
                    peek.next = Some(new_node);
                    self.length += 1;
                    return
                }
                iter_node = &mut peek.next;
            }
        }
        // Removes a node
        pub fn remove() {}
        // Prints the list
        pub fn print_list(&mut self) {
            println!("Singly inked list:");
            let mut current = &self.head;
            let mut c = 0;
            while let Some(node) = current {
                println!("{:>2}: {:<8} {:>6}", c + 1, node.name, node.score);
                current = &node.next;
                c +=1;
            }
        }

    }

}
pub fn list_adt_driver_3() {
    use singly_linked_list::{List, Node};

    let mut podium: List = List::new();
    let mut node = Node::new("Peter".to_string(), 1223);
    podium.insert(node);
    node = Node::new("Brain".to_string(), 616);
    podium.insert(node);
    podium.print_list();
    node = Node::new("Remus".to_string(), 9838);
    podium.insert(node);
    podium.print_list();
    node = Node::new("Dangus".to_string(), 34);
    podium.insert(node);
    podium.print_list();
}
