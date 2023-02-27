use crate::core::Optional;
use crate::engine::Engine;

use chrono::prelude::*;
use std::cell::RefCell;
use std::ops::Shr;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct NodeRefRc(Rc<RefCell<Node>>);

impl NodeRefRc {
  pub fn new() -> NodeRefRc {
      NodeRefRc(Rc::new(RefCell::new(Node::new())))
  }
}

#[derive(Clone, Debug, Default)]
pub struct Node {
    id: Uuid,
    upstream: Vec<Rc<RefCell<Node>>>,
    downstream: Vec<Rc<RefCell<Node>>>,
    connected: bool,
    engine: Optional<Engine>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            id: Uuid::new_v4(),
            upstream: Vec::new(),
            downstream: Vec::new(),
            connected: false,
            engine: None,
        }
    }

    pub(in crate) fn collect(&self, visited: &mut Vec<Node>) {

    }

    pub fn is_input() -> bool {
        false
    }
    // pub async fn next() -> (DateTime<Utc>, Val)
}

trait HandlesEvents {
    fn start(&self);
    fn stop(&self);
}

trait GeneratesEvents<T> {
    fn next(&self) -> (DateTime<Utc>, T);
    fn push(&self, value: T, time: DateTime<Utc>);
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node {}

impl Shr<NodeRefRc> for NodeRefRc {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        println!("Here");
        self
    }
}

/**********************************/
#[cfg(test)]
mod node_tests {
    use super::*;

    #[test]
    fn test_eq() {
        let n1 = Node::new();
        let n2 = Node::new();
        assert_eq!(n1, n1);
        assert_ne!(n1, n2);
    }
}
