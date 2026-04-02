//struct Node<V> {
//    data: V,
//    pos: Position<Node<V>> // Unique handle in the vertices list
//}
//
//struct Edge<V, E> {
//    // References to the positions in the vertices list
//    origin: Position<Node<V>>,
//    destination: Position<Node<V>>,
//    data: E,
//    pos: Position<Edge<V, E>> // Unique handle in the edges list
//}
//
//pub struct EdgeList<V, E> {
//    vertices: PosList<Node<V>>,
//    edges: PosList<Edge<V, E>>
//}
