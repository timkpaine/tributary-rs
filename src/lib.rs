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
    callable: Callable<T, U>,
}

pub struct Node<T, U> {
    inst: RefCell<NodeInst<T, U>>
}

//pub struct Graph {
//    nodes: Vec<Node>,
//}

// enum Callable<A, B, X, Y> {
//     VU(Box<dyn Fn() -> A>),
//     UV(Box<dyn Fn(A) -> ()>),
//     U(Box<dyn Fn(A) -> X>),
//     NU(Box<dyn Fn(A, B) -> X>),
//     UN(Box<dyn Fn(A) -> (X, Y)>),
//     N(Box<dyn Fn(A, B) -> (X, Y)>),
// }

pub struct Callable<T, U> (
    Box<dyn Fn(T) -> U>,
);


impl<T, U> Node<T, U> {
    pub fn new(callable: Callable<T, U>) -> Node<T, U> {
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

    pub fn from(callable: impl Fn(T) -> U + 'static) -> Node<T, U> {
        return Node::new(Callable(Box::new(callable)));
    }

    pub fn from_input(callable: impl Fn(()) -> U + 'static) -> Node<(), U> {
        return Node::new(Callable(Box::new(callable)));
    }

    pub fn from_output(callable: impl Fn(T) -> () + 'static) -> Node<T, ()> {
        return Node::new(Callable(Box::new(callable)));
    }
}

pub fn Print<T, U: std::fmt::Display>(text: &'static str, n: Node<T, U>) -> Node<U, U> {
    Node::new(
        Callable(
            Box::new(move |val: U| -> U {
                println!("{}{}", text, val);
                val
            })
        )
    )
}

impl<T, U: std::ops::Add<Output = U>> ops::Add<Node<T, U>> for Node<T, U> {
    type Output = Node<(U, U), U>;

    fn add(self, _rhs: Node<T, U>) -> Node<(U, U), U> {
        return Node::from(|vals: (U, U)| vals.0 + vals.1);
    }
}


pub fn run<T, U>(
    _to_run: Node<T, U>,
    _real_time: bool,
    _start_time: Optional<DateTime<Utc>>,
    _end_time: Optional<DateTime<Utc>>
) { }
