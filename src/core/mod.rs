mod node;
mod graph;
pub use node::*;
pub use graph::*;


pub type Optional<T> = Option<T>;

pub enum RunTarget {
    Node(NodeRefRc),
    Nodes(Vec<NodeRefRc>),
}

trait Value {}

