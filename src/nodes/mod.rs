mod ops;

pub use ops::*;

use crate::core::NodeRefRc;
use chrono::Duration;


pub fn Const<T>(value: T) -> NodeRefRc {
  NodeRefRc::new()
}

pub fn Timer<T>(value: T, interval: Duration) -> NodeRefRc {
  NodeRefRc::new()
}

pub fn Print(x: &'static str) -> NodeRefRc {
  NodeRefRc::new()
}

