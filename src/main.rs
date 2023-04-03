
use tributary::*;


fn main() {
  let n0 = Callable(Box::new(|()| 5_u32));
  let n1 = Node::from(|()| 5);
  let n2 = Node::from(|()| 6);
  let n4 = Node::from(|x: u32| -> () { });
  let n3 = n1 + n2;
  let print = Print("Out:", n3);

  // TODO
  // let out = print >> n4
  // run(out, true, None, None);
  run(print, true, None, None);
}
