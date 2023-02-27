
use tributary::*;


fn main() {
  let n1 = Node::from(|| 5);
  let n2 = Node::from(|_| 6);

  let n3 = n1 + n2;

  let out = Print("Out:", n3);

  run(out, true, None, None);
}
