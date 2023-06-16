// mod core;
// mod engine;
// mod nodes;
// pub use crate::core::*;
// pub use engine::*;
// pub use nodes::*;

// use chrono::prelude::*;
use std::boxed::Box;
use std::cell::RefCell;
use std::ops;

pub type Optional<T> = Option<T>;

pub struct NodeInst<'a, T, U> {
    callable: Callable<'a, T, U>,
}

pub struct Node<'a, T, U> {
    inst: RefCell<NodeInst<'a, T, U>>,
}

//pub struct Graph {
//    nodes: Vec<Node>,
//}

pub struct Callable<'a, T, U>(pub Box<dyn Fn(T) -> U + 'a>);

pub trait IsNode {}
impl<'a, T, U> IsNode for Node<'a, T, U> {}

impl<'a, T, U> Node<'a, T, U> {
    pub fn new(callable: Callable<'a, T, U>) -> Node<'a, T, U> {
        Node {
            inst: RefCell::new(NodeInst { callable }),
            // upstream: Vec::new(),
            // downstream: Vec::new(),
        }
    }

    pub fn from(callable: impl Fn(T) -> U + 'a) -> Node<'a, T, U> {
        Node::new(Callable(Box::new(callable)))
    }

    pub fn round(&self) -> Node<'a, f64, i64> {
        Node::new(Callable(Box::new(move |val: f64| -> i64 {
            val.round() as i64
        })))
    }
}

impl<'a, T, U> Node<'a, T, U>
where
    U: std::fmt::Display,
{
    pub fn print(&self, msg: &'static str) -> Node<'a, U, U> {
        Node::new(Callable(Box::new(move |val: U| -> U {
            println!("{}{}", msg, val);
            val
        })))
    }
}

impl<'a, T, U, X> std::ops::Add<Node<'a, X, U>> for Node<'a, T, U>
where
    U: std::ops::Add<Output = U>,
{
    type Output = Node<'a, (U, U), U>;
    fn add(self, _rhs: Node<'a, X, U>) -> Node<'a, (U, U), U> {
        return Node::from(|vals: (U, U)| vals.0 + vals.1);
    }
}

impl<'a, T, U> Node<'a, T, U>
where
    U: std::ops::Add<Output = U>,
{
    #[allow(clippy::should_implement_trait)]
    pub fn add<X>(self, _rhs: &Node<'a, X, U>) -> Node<'a, (U, U), U> {
        return Node::from(|vals: (U, U)| vals.0 + vals.1);
    }
}

impl<'a, T, U: std::ops::Add<Output = U>> ops::Add<&Node<'a, T, U>> for &Node<'a, T, U> {
    type Output = Node<'a, (U, U), U>;

    fn add(self, _rhs: &Node<'a, T, U>) -> Node<'a, (U, U), U> {
        return Node::from(|vals: (U, U)| vals.0 + vals.1);
    }
}

pub fn run<T, U>(
    _to_run: Node<T, U>,
    // _real_time: bool,
    // _start_time: Optional<DateTime<Utc>>,
    // _end_time: Optional<DateTime<Utc>>,
) {
}
