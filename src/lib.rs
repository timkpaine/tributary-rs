// mod core;
// mod engine;
// mod nodes;
// pub use crate::core::*;
// pub use engine::*;
// pub use nodes::*;

use chrono::prelude::*;
use std::boxed::Box;
use std::cell::RefCell;
use std::ops;

pub type Optional<T> = Option<T>;

pub struct NodeInst<T, U> {
    callable: Box<dyn Fn(T) -> U>,
}

pub struct Node<T, U> {
    inst: RefCell<NodeInst<T, U>>
}

//pub struct Graph {
//    nodes: Vec<Node>,
//}

impl<T, U> Node<T, U> {
    pub fn new(callable: Box<dyn Fn(T) -> U>) -> Node<T, U> {
        Node {
            inst: RefCell::new(
                NodeInst {
                    callable,
                },
            ),
            // upstream: Vec::new(),
            // downstream: Vec::new(),
        }
    }

    pub fn from(callable: impl Fn(T) -> U + 'static) -> Node<T,U> {
        return Node::new(Box::new(callable));
    }
}

pub fn Print<T, U: std::fmt::Display>(text: &'static str, n: Node<T, U>) -> Node<U, U> {
    Node::new(
        Box::new(move |val: U| -> U {
            println!("{}{}", text, val);
            val
        })
    )
}

impl<T, U: std::ops::Add<Output = U>> ops::Add<Node<T, U>> for Node<T, U> {
    type Output = Node<(U, U), U>;

    fn add(self, _rhs: Node<T, U>) -> Node<(U, U), U> {
        return Node::from(|vals: (U, U)| vals.0 + vals.1);
    }
}


pub fn run<T, U>(
    to_run: Node<T, U>,
    real_time: bool,
    start_time: Optional<DateTime<Utc>>,
    end_time: Optional<DateTime<Utc>>
) { }
