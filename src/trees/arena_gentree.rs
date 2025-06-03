pub struct Position {
    ptr: usize,
}
impl Clone for Position {
    fn clone(&self) -> Self {
        Position {
            ptr: self.ptr.clone(),
        }
    }
}

//type Position = usize;

#[derive(Debug)]
struct Node<T> {
    parent: Option<usize>,
    children: Vec<usize>,
    data: Option<T>,
    index: usize,
}

#[derive(Debug)]
pub struct GenTree<T> {
    arena: Vec<Node<T>>,
    size: usize,
}
impl<T> GenTree<T> {
    /** Creates a new GenTree */
    pub fn new() -> GenTree<T> {
        GenTree {
            arena: Vec::with_capacity(100),
            size: 0,
        }
    }

    /** Gets a "pointer" to the tree's root node */
    //pub fn root(&self) -> Position {
    //    Position {
    //        ptr: 0
    //    }
    //}
    pub fn root(&self) -> usize {
        0
    }

    //pub fn cursor_mut(&mut self) -> CursorMut<'_, T> {}
    //pub fn cursor_from(&mut self, position: Position<T>) -> CursorMut<'_, T> {}

    pub fn depth(&mut self, _node: Position) -> usize {
        0
    }
    pub fn height(&mut self, _node: Position) -> usize {
        0
    }

    // All of the tings implemented on CursorMut
    ////////////////////////////////////////////

    /** Returns true if the cursor's parent is None */
    //pub fn is_root(&self) -> bool {
    //    self.arena[0].parent.is_none()
    //}

    pub fn is_some(&self, position: usize) -> bool {
        self.arena[position].data.is_some()
    }

    pub fn is_none(&self, position: usize) -> bool {
        self.arena[position].data.is_none()
    }

    /** Returns the number of children for a Node at the given Position */
    pub fn num_children(&self, position: usize) -> usize {
        self.arena[position].children.len()
    }

    /** Returns an immutable reference to the data at the given Position, if Some */
    //pub fn get_data(&self) -> Option<Ref<'_, T>> {}
    pub fn get_data(&self, position: usize) -> Option<&T> {
        if let Some(val) = &self.arena[position].data {
            Some(&val)
        } else {
            None
        }
    }

    /** Adds a child to the given Position */
    pub fn add_child(&mut self, position: usize, data: T) -> usize {
        // Ensures proper index
        let index = self.arena.len();

        // Adds the node to the arena
        self.arena.push(Node {
            parent: if index == 0 { None } else { Some(position) },
            children: Vec::new(),
            data: Some(data),
            index,
        });

        // Push the new Node's index to the parent's list of children
        if self.arena.len() == 1 {
        } else {
            self.arena[position].children.push(index)
        };

        // Increase the size of the tree and returns the new Node's index
        self.size += 1;
        index
    }

    /** Returns a reference to the list of child positions for the given Position */
    pub fn children(&self, position: usize) -> &Vec<usize> {
        &self.arena[position].children
    }

    /** Deletes a Node at the given position, moves all of the deleted Node's children to the
    deleted Node's parent, if Some */
    pub fn delete(&mut self, position: Position) -> Option<T> {
        // Gets the data out of the deleted Node, leaving None in its place
        let data = self.arena[position.ptr].data.take();

        // Gets the deleted Node's parent's position
        let parent_pos = self.arena[position.ptr].parent.clone();

        // Move all of the deleted Node's children up a generation, if theres a parent
        if let Some(parent_node) = parent_pos {
            for position in self.arena[position.ptr].children.clone().into_iter() {
                self.arena[parent_node].children.push(position)
            }
            data
        }
        // If the deleted Node has no parent (root), just delete the Node's data
        else {
            self.arena[position.ptr].data = None;
            data
        }
    }

    /***/
    //pub fn current(&self) -> &Position<T> {}

    /***/
    //pub fn jump(&mut self, new: &Position<T>) {}

    /***/
    pub fn ascend(&mut self, position: usize) -> Result<usize, String> {
        if let Some(parent) = self.arena[position].parent {
            Ok(parent)
        } else {
            Err("Cannot ascend past the root node".to_string())
        }
    }

    //pub fn jump(&self)
}

mod builder {

    use super::*;

    #[allow(unused)]
    #[derive(Debug)]
    pub struct Heading {
        pub level: usize,
        pub title: String,
    }

    pub fn construct(mut cur_level: usize, data: Vec<Heading>) -> GenTree<Heading> {
        // Instantiates a Tree with a generic root and traversal positioning
        let mut tree: GenTree<Heading> = GenTree::<Heading>::new();
        let mut cursor = tree.root(); // Sets cursor to tree.root

        // Constructs tree from Vec<T>
        for node in data {
            let title_level = node.level;

            // Case 1: Adds a child to the current parent
            if title_level == cur_level + 1 {
                cursor = tree.add_child(cursor, node);
                cur_level += 1;
            }
            // Case 2: Adds a child with multi-generational skips
            else if title_level > cur_level {
                let diff = title_level - cur_level;
                for _ in 1..diff {
                    //let empty = Heading::new("[]".to_string(), 0);
                    let empty = Heading {
                        title: "[]".to_string(),
                        level: 0,
                    };
                    cursor = tree.add_child(cursor, empty);
                    cur_level += 1;
                }
                cursor = tree.add_child(cursor, node);
                cur_level += 1;
            }
            // Case 3: Adds sibling to current parent
            else if title_level == cur_level {
                cursor = tree.ascend(cursor).expect("Oopsie");
                cursor = tree.add_child(cursor, node);
            }
            // Case 4: Adds a child to the appropriate ancestor,
            // ensuring proper generational skips
            else {
                let diff = cur_level - title_level;
                for _ in 0..=diff {
                    cursor = tree.ascend(cursor).expect("Dang!");
                    cur_level -= 1;
                }
                cursor = tree.add_child(cursor, node);
                cur_level += 1;
            }
        }
        tree
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::trees::arena_gentree::builder::{construct, Heading};

    #[test]
    fn one() {
        let tree_vec = vec![
            Heading {
                level: 2,
                title: "Landlocked".to_string(),
            },
            Heading {
                level: 3,
                title: "Switzerland".to_string(),
            },
            Heading {
                level: 4,
                title: "Geneva".to_string(),
            },
            Heading {
                level: 5,
                title: "Old Town".to_string(),
            },
            Heading {
                level: 6,
                title: "Cathédrale Saint-Pierre".to_string(),
            },
            Heading {
                level: 3,
                title: "Bolivia".to_string(),
            },
            Heading {
                level: 6,
                title: "Puerta del Sol".to_string(),
            },
            Heading {
                level: 6,
                title: "Puerta de la Luna".to_string(),
            },
            Heading {
                level: 2,
                title: "Islands".to_string(),
            },
            Heading {
                level: 3,
                title: "Marine".to_string(),
            },
            Heading {
                level: 4,
                title: "Australia".to_string(),
            },
            Heading {
                level: 3,
                title: "Fresh Water".to_string(),
            },
        ];

        // Constructs tree ignoring the first heading
        let mut tree: GenTree<Heading> = construct(0, tree_vec);
        // For some reason this statement causes Miri to take much longer than necessary!
        // As a result, this cfg prevents it from running with Miri
        //#[cfg(not(miri))]
        eprintln!("{:#?}", tree);
        let mut cursor = tree.root();

        // Tests that the root contains data
        //assert!(!tree.get_data(cursor).is_some());

        // Tests that root is empty with is_some() and is_none()
        //assert!(!tree.is_some(cursor));
        //assert!(tree.is_none(cursor));

        // Tests num_children()
        assert_eq!(tree.num_children(cursor), 2); // Root has [Landlocked, Islands]

        // Tests children() and get_data()
        let kids = tree.children(cursor);
        let mut kids_iter = kids.iter();

        // Steps through the root's children
        cursor = *kids_iter.next().unwrap();
        let data = tree.get_data(cursor).unwrap();
        assert_eq!(*data.title, "Landlocked".to_string());
        cursor = *kids_iter.next().unwrap();
        let data = tree.get_data(cursor).unwrap();
        assert_eq!(*data.title, "Islands".to_string());

        // Jumps down a generation to Islands' kids [Marine, Fresh Water]
        let new_kids = tree.children(cursor);
        kids_iter = new_kids.iter();
        cursor = *kids_iter.next().unwrap();
        let data = tree.get_data(cursor).unwrap();
        assert_eq!(*data.title, "Marine".to_string());

        // Jumps down another generation, for fun
        let new_kids = tree.children(cursor); // Gets cursor's chidlren
        kids_iter = new_kids.iter(); // Creates an iterator
        cursor = *kids_iter.next().unwrap(); // Moves to first child
        let data = tree.get_data(cursor).unwrap();
        assert_eq!(*data.title, "Australia".to_string());

        // Tests ascend()
        let parent = tree.ascend(cursor).ok().unwrap(); // Marine
        let data = tree.get_data(parent).unwrap();
        assert_eq!(*data.title, "Marine".to_string());
        cursor = parent;
        let parent = tree.ascend(cursor).ok().unwrap(); // Islands
        let data = tree.get_data(parent).unwrap();
        assert_eq!(*data.title, "Islands".to_string());
        cursor = parent;
        cursor = tree.ascend(cursor).ok().unwrap(); // []
        assert!(tree.ascend(cursor).is_err()); // Cannot ascend() past root

        // Descends to Islands to test delete()
        let kids = tree.children(cursor); // Gets cursor's chidlren
        let mut kids_iter = kids.iter(); // Creates an iterator
                                         //cursor.jump(kids_iter.next().unwrap()); // Moves to Landlocked
                                         //cursor.jump(kids_iter.next().unwrap()); // Moves to Islands
                                         //{
                                         //    let data = cursor.get_data().unwrap();
                                         //    assert_eq!(*data.title, "Islands".to_string());
                                         //}

        //// Tests delete()
        //// Creates placeholder Heading
        //let mut deleted = Heading { title: String::new(), level: 0 };
        //// Iterates through the child position's under the cursor
        //// looking for a matching Heading; Once found, jumps to that position,
        //// and deletes the Heading; The delete() operation automatically jumps
        //// the cursor to the parent of the deleted position
        //for position in cursor.children().iter() {
        //    if position.get_data().unwrap().title == "Marine".to_string() {
        //        cursor.jump(&position);
        //        deleted = cursor.delete().unwrap();
        //    }
        //}
        //// Tests that the correct Heading was deleted
        //assert_eq!(deleted.level, 3);
        //assert_eq!(deleted.title, "Marine".to_string());

        //// Tests that the cursor got bumped up to Islands
        //let data = cursor.get_data().unwrap();
        //assert_eq!(data.title, "Islands".to_string());

        //// Tests that the Islands node has the correct children
        //let mut kids = Vec::new();
        //assert_eq!(cursor.children().len(), 2);
        //for child in cursor.children().iter() {
        //    let title = child.get_data().unwrap().title.clone();
        //    kids.push(title)
        //}
        //assert_eq!(kids, ["Fresh Water".to_string(), "Australia".to_string()]);
    }

    #[test]
    fn dangle() {
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
        let mut _pos: usize = outer_tree.root();

        {
            let inner_tree: GenTree<Heading> = construct(0, two);
            _pos = inner_tree.root();
        } // inner_tree dropped here

        // No UB because Vec gets dropped
        //let _oopsie = inner_tree.get_data(_pos);
    }
}
