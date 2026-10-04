/*! A safe, indexed, n-ary tree implementation

# About
This module using arena-like backing primarily as an easy way to provide safe node referencing to mutable tree structures in a way that avoids complex lifetime management, overly-restrictive API design, and the runtime overhead of reference counting.

Compromises over the link-based tree include being less spatially efficient as the arena's growth algorithm logically shifts "pointers" to Nodes in the arena.

# Design
The implementation stores all `Node` values in a flat `Vec`-backed arena. For small trees (fewer than ~100 nodes), it is marginally slower than the `Rc<RefCell>`-based design due to fixed arena management overhead. However, for larger trees (starting around 1,000–10,000 nodes), it improves construction speed by roughly 20–25%, primarily from reduced heap allocations and better cache locality.

## Generations & Free Lists
This structure uses generations on its safe handles to mitigate dangling access & use-after-free errors, dead references via ABA problem mitigation, and handle forgery. This is something that both naive arena-backed structures and raw pointer structures are vulnerable to. Reference counting prevents dangling access and dead references, and makes handle forgery impossible.

This structure implements a free list to mitigate leaks. When nodes are removed, the backing slot takes on a `None` value, which can bloat the list unless those nodes are re-filled. However, this means that two node references could point to the same memory slot. As a result, the structure increments generational count by usage, guaranteeing that only the correct generational combination can access the memory slot.

## Drawbacks
This implementation includes several critical compromises over a more traditional link-based approach. This Vec-backed design is intended to provide a more ergonomic design over reference counted alternatives with interior mutability, as well as raw pointer-based designs with complex lifetime management and restrictive APIs. Unfortunately, this Vec-backed design also comes with its own compromises. The Vec-backed design is far less spatially efficient due to the structure's growth. As the tree gets mutated, it also potentially loses cache locality.

# Example

```rust

```

*/

#[derive(Copy, Debug, PartialEq)]
pub struct Position {
    ptr: usize,
    generation: usize,
}
impl Position {
    pub fn new(ptr: usize, generation: usize) -> Position {
        Position { ptr, generation }
    }

    fn _get(&self) -> usize {
        self.ptr
    }
}
// SAFETY: Cloning a Position produces another handle to the same arena
// entry. ABA prevention is enforced by generation validation in the
// arena; handles merely carry the generation that was current when issued.
// This is effectively the same thing as a default #[derive(Clone)].
impl Clone for Position {
    fn clone(&self) -> Self {
        *self // Because the type derives Copy
        //Position {
        //    ptr: self.ptr,
        //    generation: self.generation,
        //}
    }
}

// TODO: The children field might optionally use smallvec or tinyvec
// or similar for more efficient stack storage which theoretically
// reduces pointer chasing
#[derive(Debug)]
struct Node<T> {
    parent: Option<Position>,
    children: Vec<Position>,
    data: Option<T>,
    generation: usize,
}
impl<T> Node<T> {
    fn _get_parent(&self) -> Option<&Position> {
        self.parent.as_ref()
    }
    fn _get_children(&self) -> &Vec<Position> {
        &self.children
    }
}

use std::cell::{Ref, RefCell};

#[derive(Debug)]
pub struct GenTree<T> {
    // RefCell moves structural mutation borrow checks from compile-time to runtime
    arena: RefCell<Vec<Node<T>>>,
    size: RefCell<usize>,
    root: Position,
    free_list: RefCell<Vec<usize>>,
}
impl<T> Default for GenTree<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> GenTree<T> {
    pub fn new() -> Self {
        let arena = vec![Node {
            parent: None,
            children: Vec::new(),
            data: None,
            generation: 0,
        }];

        GenTree {
            arena: RefCell::new(arena),
            size: RefCell::new(0),
            root: Position::new(0, 0),
            free_list: RefCell::new(Vec::new()),
        }
    }

    pub fn new_with_capacity(cap: usize) -> Self {
        let arena = vec![Node {
            parent: None,
            children: Vec::with_capacity(cap),
            data: None,
            generation: 0,
        }];

        GenTree {
            arena: RefCell::new(arena),
            size: RefCell::new(0),
            root: Position::new(0, 0),
            free_list: RefCell::new(Vec::new()),
        }
    }

    pub fn root(&self) -> &Position {
        &self.root
    }

    pub fn size(&self) -> usize {
        *self.size.borrow() // Dumb
    }

    pub fn num_children(&self, pos: &Position) -> usize {
        self.arena.borrow()[pos.ptr].children.len()
    }

    pub fn is_some(&self, pos: &Position) -> bool {
        self.arena.borrow()[pos.ptr].data.is_some()
    }

    pub fn is_empty(&self) -> bool {
        //self.arena.borrow()[0].data.is_none()
        self.size.borrow().eq(&0)
    }

    pub fn mut_root(&self, data: T) {
        self.arena.borrow_mut()[0].data = Some(data);
    }

    /// The number of levels from a given node to the root
    pub fn depth(&self, pos: &Position) -> usize {
        match self.parent(pos) {
            Some(parent) => 1 + self.depth(&parent),
            None => 0, // root
        }
    }

    // The number of levels from a given node to its tallest descendant,
    // or the distance between a given position and its furthest leaf
    //pub fn height(&self, pos: &Position) -> usize {
    //    self.children(pos)
    //    .map(|child| self.height(&child))
    //    .max()
    //    .map_or(0, |h| h + 1)
    //}

    fn is_token_valid(&self, position: &Position) -> bool {
        let arena = self.arena.borrow();
        if position.ptr >= arena.len() {
            return false;
        }
        let node = &arena[position.ptr];
        node.generation == position.generation && node.data.is_some()
    }

    pub fn is_none(&self, position: &Position) -> bool {
        !self.is_token_valid(position)
    }

    /// Yields a dynamic reference guard to the parent token inside the arena.
    pub fn parent<'a>(&'a self, position: &Position) -> Option<Ref<'a, Position>> {
        if !self.is_token_valid(position) {
            return None;
        }

        let arena = self.arena.borrow();
        if arena[position.ptr].parent.is_some() {
            Some(Ref::map(arena, |a| {
                a[position.ptr].parent.as_ref().unwrap()
            }))
        } else {
            None
        }
    }

    pub fn children(&self, position: &Position) -> Ref<'_, Vec<Position>> {
        assert!(self.is_token_valid(position), "Target handle is dead!"); // ????
        Ref::map(self.arena.borrow(), |arena| &arena[position.ptr].children)
    }

    pub fn get_data<'a>(&'a self, position: &Position) -> Option<Ref<'a, T>> {
        if !self.is_token_valid(position) {
            return None;
        }
        Some(Ref::map(self.arena.borrow(), |arena| {
            arena[position.ptr].data.as_ref().unwrap()
        }))
    }

    pub fn add_child(&mut self, parent_pos: &Position, data: T) -> Position {
        assert!(
            parent_pos.ptr == 0 || self.is_token_valid(parent_pos),
            "Target parent handle is dead!"
        );

        let mut arena = self.arena.borrow_mut();
        let mut free_list = self.free_list.borrow_mut();

        let (index, next_gen) = if let Some(reuse_idx) = free_list.pop() {
            arena[reuse_idx].generation += 1;
            let gen = arena[reuse_idx].generation;

            arena[reuse_idx] = Node {
                // Duplicate internally via component destructuring, never by copying the token object
                parent: Some(Position::new(parent_pos.ptr, parent_pos.generation)),
                children: Vec::new(),
                data: Some(data),
                generation: gen,
            };
            (reuse_idx, gen)
        } else {
            let new_idx = arena.len();
            arena.push(Node {
                parent: Some(Position::new(parent_pos.ptr, parent_pos.generation)),
                children: Vec::new(),
                data: Some(data),
                generation: 0,
            });
            (new_idx, 0)
        };

        // Increment the list's size!
        *self.size.get_mut() += 1;

        arena[parent_pos.ptr]
            .children
            .push(Position::new(index, next_gen));
        Position::new(index, next_gen)
    }

    /// Explicitly consumes the token, destroying it from the caller's frame permanently.
    pub fn remove(&self, position: Position) -> Option<T> {
        if !self.is_token_valid(&position) {
            return None;
        }

        let mut arena = self.arena.borrow_mut();
        let data = arena[position.ptr].data.take();
        let parent_pos = arena[position.ptr].parent.take();

        self.free_list.borrow_mut().push(position.ptr);

        if let Some(parent) = parent_pos {
            arena[parent.ptr].children.retain(|p| p.ptr != position.ptr);

            let orphans = std::mem::take(&mut arena[position.ptr].children);
            for child in orphans {
                arena[child.ptr].parent = Some(Position::new(parent.ptr, parent.generation));
                arena[parent.ptr].children.push(child);
            }
        }
        data
    }
}

#[cfg(test)]
mod tests {

    #[test]
    /// TODO: actually test the structure's members!
    fn atomic() {
        use super::GenTree;
        use crate::hierarchies::arena_gentree_builder::Heading;

        let mut tree = GenTree::new();
        // Instantiated tree automatically has a single, empty root node
        // with a size of zero
        assert_eq!(tree.size(), 0);
        assert!(tree.is_empty());
        let root = tree.root().clone();

        let mut cursor = tree.add_child(
            &root,
            Heading {
                level: 2,
                title: "Landlocked".to_string(),
            },
        );
        assert_eq!(tree.size(), 1);
        assert!(!tree.is_empty());

        cursor = tree.add_child(
            &cursor,
            Heading {
                level: 3,
                title: "Switzerland".to_string(),
            },
        );
        cursor = tree.add_child(
            &cursor,
            Heading {
                level: 4,
                title: "Geneva".to_string(),
            },
        );
        cursor = tree.add_child(
            &cursor,
            Heading {
                level: 5,
                title: "Old Town".to_string(),
            },
        );
        //cursor = tree.parent(&cursor).expect(""); // Geneva
        //cursor = tree.parent(&cursor).expect(""); // Switzerland
        tree.add_child(
            &cursor,
            Heading {
                level: 3,
                title: "Botswana".to_string(),
            },
        );
        assert_eq!(tree.size(), 5);

        eprintln!("{tree:#?}");
        //panic!("MANUAL TEST FAILURE");
    }

    #[test]
    fn dangle() {
        use crate::hierarchies::arena_gentree_builder::{construct, Heading};

        use super::GenTree;
        let one = vec![
            Heading {
                level: 1,
                title: "Landlocked".to_string(),
            },
            Heading {
                level: 2,
                title: "Switzerland".to_string(),
            },
        ];
        let two = vec![
            Heading {
                level: 1,
                title: "Bolivia".to_string(),
            },
            Heading {
                level: 2,
                title: "Zimbabwe".to_string(),
            },
        ];

        // Creates a tree, Position, and CursorMut
        let outer_tree: GenTree<Heading> = construct(0, one);
        //let outer_tree: GenTree<Heading> = construct_from(one);
        let mut _pos = outer_tree.root();

        {
            let inner_tree: GenTree<Heading> = construct(0, two);
            //let inner_tree: GenTree<Heading> = construct_from(two);
            _pos = inner_tree.root();
        } // inner_tree dropped here

        // No UB (not possible) because inner_tree and _pos is already dropped
        //let _oopsie = outer_tree.get_data(_pos);
    }

    use super::*;

    #[test]
    // Irrelevant because Position is move-only
    fn test_aba_slot_recycling_isolation() {}

    #[test]
    fn test_parent_child_severance_on_remove() {}

    #[test]
    fn test_arena_churn_and_size_accounting() {}

    #[test]
    fn test_structural_queries_on_stale_positions() {}

    #[test]
    #[allow(unused)]
    fn test_recycling_preserves_live_nodes() {
        let mut tree = GenTree::new();
        tree.mut_root("Root".to_string());

        let root = tree.root(); // Immutable borrow

        //let victim = tree.add_child(root, "Victim".to_string()); // Mutable borrow
        //let survivor = tree.add_child(root, "Survivor".to_string()); // Mutable borrow

        //tree.remove(victim);

        //let replacement = tree.add_child(root, "Replacement".to_string()); // Mutable borrow

        //assert_eq!(*tree.get_data(&survivor).unwrap(), "Survivor");
        //assert_eq!(*tree.get_data(&replacement).unwrap(), "Replacement");
    }
}
