
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


  // some random tests
  let n4 = Node::from(|x: u32| -> u32 { return x; });
  let n4 = Node::from(|(x, y)| -> (u32, u32) {return (x, y);} );
  let n4 = Node::from(|(x)| -> (u32, u32) {return (x, x);} );
  let n4 = Node::from(|()| -> (u32, u32) {return (4, 5);} );
}
