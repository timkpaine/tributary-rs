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

pub struct NodeInst<'a, T, U> {
    callable: Callable<'a, T, U>,
}

pub struct Node<'a, T, U> {
    inst: RefCell<NodeInst<'a, T, U>>
}

//pub struct Graph {
//    nodes: Vec<Node>,
//}

pub struct Callable<'a, T, U> (
    pub Box<dyn Fn(T) -> U + 'a>,
);

impl<'a, T, U> Node<'a, T, U> {
    pub fn new(callable: Callable<'a, T, U>) -> Node<T, U> {
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

    pub fn from(callable: impl Fn(T) -> U + 'a) -> Node<'a, T, U> {
        return Node::new(Callable(Box::new(callable)));
    }

    pub fn from_input(callable: impl Fn(()) -> U + 'a) -> Node<'a, (), U> {
        return Node::new(Callable(Box::new(callable)));
    }

    pub fn from_output(callable: impl Fn(T) -> () + 'a) -> Node<'a, T, ()> {
        return Node::new(Callable(Box::new(callable)));
    }
}

pub fn Print<'a, T, U: std::fmt::Display>(text: &'static str, n: Node<'a, T, U>) -> Node<'a, U, U> {
    Node::new(
        Callable(
            Box::new(move |val: U| -> U {
                println!("{}{}", text, val);
                val
            })
        )
    )
}

impl<'a, T, U: std::ops::Add<Output = U>> ops::Add<Node<'a, T, U>> for Node<'a, T, U> {
    type Output = Node<'a, (U, U), U>;

    fn add(self, _rhs: Node<'a, T, U>) -> Node<'a, (U, U), U> {
        return Node::from(|vals: (U, U)| vals.0 + vals.1);
    }
}


pub fn run<T, U>(
    _to_run: Node<T, U>,
    _real_time: bool,
    _start_time: Optional<DateTime<Utc>>,
    _end_time: Optional<DateTime<Utc>>
) { }
