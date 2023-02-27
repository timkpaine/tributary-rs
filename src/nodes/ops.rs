use crate::core::NodeRefRc;
use std::ops;


impl ops::Add<NodeRefRc> for NodeRefRc {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
      NodeRefRc::new()
  }
}

