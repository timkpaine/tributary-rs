
use tributary::*;


fn main() {
  let n0 = Callable(Box::new(|()| 5_u32));
  let n1 = Node::from_input(|()| 5);
  let n2 = Node::from_input(|()| 6);

  let n3 = n1 + n2;

  let out = Print("Out:", n3);

  run(out, true, None, None);
}
