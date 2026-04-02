use std::cmp::PartialEq;

pub struct Node<T, M> {
    data: T,
    // Simple adjacency list structuring
    links: Vec<Link<M>>,
    // Adjacency map structring provides
    // O(1) access for the get_edge(u, v) operation
    //edges: std::collections::HashMap<Link<'a, T, X>>
}
impl<T, M> Node<T, M> {
    fn new(data: T) -> Node<T, M> {
        Node {
            data,
            links: Vec::new(),
        }
    }
}
impl<T, M> PartialEq for Node<T, M>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

#[allow(dead_code)]
pub struct Link<M> {
    //destination: &'a Node<'a, T, X>,
    destination: usize,
    metadata: Option<M>, // For weighted graphs
}
impl<M> Link<M> {
    //fn new(destination: &'a Node<'a, T, X>, metadata: Option<X>) -> Self {
    fn new(destination: usize, metadata: Option<M>) -> Self {
        Link {
            destination,
            metadata,
        }
    }
}
//impl<M: PartialEq> PartialEq for Link<M> {
//    fn eq(&self, other: &Self) -> bool {
//        self.destination == other.destination &&
//            self.metadata == other.metadata
//    }
//}

/// Represents a pointer bag design for an adjacency list graph representation.
/// Nodes are identified by stable `Vec` indices for the lifetime of the graph.
//struct AdjacencyList<'a, T, X>(Vec<Node<'a, T, X>>);
pub struct AdjacencyList<T, M> {
    bag: Vec<Node<T, M>>,
}
impl<T, M> Default for AdjacencyList<T, M>
where
    T: PartialEq,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T, M> AdjacencyList<T, M>
where
    T: PartialEq,
{
    // Factories
    ////////////

    /// Creates a new, empty graph
    pub fn new() -> Self {
        //AdjacencyList(Vec::new())
        AdjacencyList { bag: Vec::new() }
    }

    // Mutators
    ///////////

    /// Creates and adds a new vertex for the specified data
    pub fn add_node(&mut self, data: T) -> usize {
        let node = Node::new(data);
        self.bag.push(node);
        self.bag.len() - 1
    }

    /// Creates and adds a new edge from an originating vertex to a
    /// destination vertex with an optional metadata element `M` such
    /// as a weight. Origin and destination arguments are positional
    /// for directed graphs.
    pub fn add_directed_link(&mut self, origin: usize, destination: usize, metadata: Option<M>) {
        // Create the new Link
        let link = Link::new(destination, metadata);

        // Add the link to the origin node's list of links
        self.bag[origin].links.push(link);
    }

    /// Creates and adds a new _undirected_ edge from one node to another
    /// node with an optional metadata element `M` such
    /// as a weight.
    pub fn add_undirected_link(&mut self, first: usize, second: usize, metadata: Option<M>)
    where
        M: Clone,
    {
        // Create the new Link
        let link = Link::new(second, metadata.clone());

        // Add the link to the origin node's list of links
        self.bag[first].links.push(link);

        // Create the new Link
        let link = Link::new(first, metadata);

        // Add the link to the destination node's list of links
        self.bag[second].links.push(link);
    }

    //remove_vertex(v)` Removes vertex `v` and all of its incident edges
    //remove_edge(e)` Removes edge `e`

    // Accessors
    ////////////

    /// Returns a reference to a node for a given index.
    pub fn get_node(&self, index: usize) -> Option<&Node<T, M>> {
        // Handles len check internally and returns Option by default
        // because some APIs are better than others
        self.bag.get(index)
    }

    #[allow(dead_code)]
    /// Locates the node's stable index by value in _O(n)_ time.
    fn get_node_index(&self, node: &Node<T, M>) -> Option<usize> {
        for (i, val) in self.bag.iter().enumerate() {
            if val.data == node.data {
                return Some(i);
            }
        }
        None
    }

    /// Returns the number of nodes in the graph
    pub fn num_nodes(&self) -> usize {
        self.bag.len()
    }

    /// Returns the number of unique links in the graph
    pub fn num_links(&self) -> usize {
        self.links().len()
    }

    // /// Returns the edge between vertices `u` and `v`, this can be positional for directed graphs, and symmetric for undirected graphs
    //pub fn get_link(&self, origin: usize, destination: usize) {}

    // /// Returns an array containing the two endpoint
    // /// vertides of edge `e`, this can be positional for directed graphs,
    // /// and symmetric for undirected graphs
    //pub fn get_nodes(&self, link: usize) -> (&Node<T, M>, &Node<T, M>) {}

    //opposite(v, e)` Returns the pair vertex of an edge `e` incident to vertex `v`
    //is_adjacent(u, v)` Returns a Boolean to indicate if vertices `u` and `v` are adjacent
    //out_degree(v)` Returns the number of outgoing edges from vertex `v`
    //in_degree(v)` Returns the number of incoming edges from vertex `v`

    /// Returns an iterable collection over all vertices in the graph
    pub fn nodes(&self) -> &Vec<Node<T, M>> {
        &self.bag
    }

    /// Returns an iterator over all vertices in the graph
    pub fn nodes_iter(&self) -> impl Iterator<Item = &Node<T, M>> {
        self.bag.iter()
    }

    // Returns an iteraterable collection over
    //outgoing edges from vertex `v`
    pub fn out_links(&self, index: usize) -> &[Link<M>] {
        &self.bag[index].links
    }

    /// Returns an iterator over all outgoing links for a given node
    pub fn out_links_iter(&self, index: usize) -> impl Iterator<Item = &Link<M>> {
        self.bag[index].links.iter()
    }

    /// Returns an iterator over incoming links from a given node
    pub fn in_links(&self, index: usize) -> Vec<&Link<M>> {
        let mut list = Vec::new();
        for node in self.bag.iter() {
            for link in node.links.iter() {
                if link.destination == index {
                    list.push(link)
                }
            }
        }
        list
    }

    /// Returns an iterator over all incoming links to a given node
    pub fn in_links_iter(&self, index: usize) -> impl Iterator<Item = (&Node<T, M>, &Link<M>)> {
        self.bag.iter().flat_map(move |node| {
            node.links
                .iter()
                .filter(move |link| link.destination == index)
                .map(move |link| (node, link))
        })
    }

    /// Returns an iteratable collection over all links in the graph as
    /// { (u, (u → v)) | u ∈ V, (u → v) ∈ Adj[u] }
    pub fn links(&self) -> Vec<(&Node<T, M>, &Link<M>)> {
        let mut links: Vec<(&Node<T, M>, &Link<M>)> = Vec::new();
        for node in self.bag.iter() {
            for link in node.links.iter() {
                links.push((node, link))
            }
        }
        links
    }

    /// Returns an iterator over all links in the graph
    pub fn links_iter(&self) -> impl Iterator<Item = (&Node<T, M>, &Link<M>)> {
        self.bag
            .iter()
            .flat_map(|node| node.links.iter().map(move |link| (node, link)))
    }
}

// (Explicit) Iterator Structs
//////////////////////////////

//pub struct Links {}
//impl Links {}
//impl Iterator for Links{
//    type Item = ;
//    fn next(&mut self) -> Option<Self::Item> {}
//}
//pub struct Union<'a, K> {
//    // Used "raw" for its "free" operations
//    lhs: &'a probing_hash_table::HashMap<K, ()>,
//    // Create explicit iterators outside of the next() implementation
//    lhs_iter: probing_hash_table::Keys<'a, K, ()>,
//    rhs_iter: probing_hash_table::Keys<'a, K, ()>,
//}
//impl<'a, K> Union<'a, K> {
//    // Constructor that takes map references to create iterators outside of the next()
//    // implementation
//    fn build(
//        lhs: &'a probing_hash_table::HashMap<K, ()>,
//        rhs: &'a probing_hash_table::HashMap<K, ()>,
//    ) -> Union<'a, K>
//    where
//        K: Debug + Eq + Hash + PartialEq,
//    {
//        Union {
//            lhs_iter: lhs.keys(),
//            rhs_iter: rhs.keys(),
//            lhs,
//        }
//    }
//}
//impl<'a, K> Iterator for Union<'a, K>
//where
//    K: Debug + Eq + Hash,
//{
//    type Item = &'a K;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        // Yield elements from lhs iterator
//        if let Some(key) = self.lhs_iter.next() {
//            return Some(key);
//        }
//
//        // Then yield only unique elements from the rhs iterator
//        self.rhs_iter.by_ref().find(|&key| !self.lhs.contains(key))
//        //while let Some(key) = self.rhs_iter.next() {
//        //    if !self.lhs.contains(key) {
//        //        return Some(key);
//        //    }
//        //}
//        //None
//    }
//}

#[cfg(test)]
mod graph_tests {
    use super::*;

    #[test]
    fn one() {
        // 1) Build the following edgeless (unconnected) graph:
        //
        //   [A] [B] [C] [D]
        //
        //   [E] [F] [G] [H]
        //
        //   [I] [J] [K] [L]
        //
        //   [M] [N] [O] [P]

        let mut graph = AdjacencyList::<&str, usize>::new();
        for name in [
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P",
        ] {
            graph.add_node(name);
        }

        // 2) Add links to create the following connected, undirected graph:
        //
        //   [A] --- [B] --- [C] --- [D]
        //   |  \     |       |     / |
        //   |   \    |       |    /  |
        //   |     \  |       |  /    |
        //   |      \ |       | /     |
        //   [E] --- [F]     [G]     [H]
        //   |      /       / | \     |
        //   |     /       /  |  \    |
        //   |   /       /    |    \  |
        //   |  /       /     |     \ |
        //   [I] --- [J] --- [K]     [L]
        //   |  \           / |       |
        //   |   \         /  |       |
        //   |     \     /    |       |
        //   |      \   /     |       |
        //   [M] --- [N]     [O]     [P]

        graph.add_undirected_link(0, 1, None);
        graph.add_undirected_link(1, 2, None);
        graph.add_undirected_link(2, 3, None);
        graph.add_undirected_link(3, 6, None);
        graph.add_undirected_link(4, 0, None);
        graph.add_undirected_link(4, 5, None);
        graph.add_undirected_link(5, 0, None);
        graph.add_undirected_link(5, 1, None);
        graph.add_undirected_link(5, 8, None);
        graph.add_undirected_link(6, 2, None);
        graph.add_undirected_link(6, 9, None);
        graph.add_undirected_link(6, 11, None);
        graph.add_undirected_link(7, 3, None);
        graph.add_undirected_link(8, 4, None);
        graph.add_undirected_link(8, 12, None);
        graph.add_undirected_link(9, 8, None);
        graph.add_undirected_link(10, 9, None);
        graph.add_undirected_link(10, 6, None);
        graph.add_undirected_link(10, 14, None);
        graph.add_undirected_link(11, 7, None);
        graph.add_undirected_link(11, 15, None);
        graph.add_undirected_link(12, 13, None);
        graph.add_undirected_link(13, 8, None);
        graph.add_undirected_link(13, 10, None);

        // 3) Traverse the graph with DFS:
        //
        //   [A] --> [B] --> [C] --> [D]
        //   ^  ^     ^       ^     / ^
        //   |   \    |       |    /  |
        //   |     \  |       |  /    |
        //   |      \ |       | v     |
        //   [E] --> [F]     [G]     [H]
        //   ^      /       / ^ \     ^
        //   |     /       /  |  \    |
        //   |   /       /    |    \  |
        //   |  v       v     |     v |
        //   [I] <-- [J] <-- [K]     [L]
        //   |  ^            ^|       |
        //   |   \          / |       |
        //   |     \      /   |       |
        //   v      \   /     v       v
        //   [M] --> [N]     [O]     [P]
        //
        // With the following traversal order:
        // A -> B (FE),
        // B -> C (FE),
        // C -> D (FE),
        // D -> G (FE),
        // G -> C (FE),

        let _a: [usize; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    }
}
