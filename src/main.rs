mod streaming;
use streaming::Node;

fn foo() -> &'static str {
    "world"
}

fn main() {
    let n = Node { func: foo };
    println!("Hello {}!", n.call());
}
