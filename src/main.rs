use rand::Rng;

use tributary::*;

fn main() {
    let random: Node<'_, (), f64> = Node::from(|()| rand::thread_rng().gen::<f64>());
    let rounded: Node<'_, f64, i64> = random.round();
    let constant: Node<'_, (), i64> = Node::from(|()| 5);
    let constant2: Node<'_, (), i64> = Node::from(|()| 2);
    let result: Node<'_, (i64, i64), i64> = rounded + constant.add(&constant2);
    let printed: Node<'_, i64, i64> = result.print("result:");

    run(printed);

    // let mut dyn_list: Vec<Box<dyn IsNode>> = vec![];
    // dyn_list.push(Box::new(n1));
    // dyn_list.push(Box::new(n2));
    // dyn_list.push(Box::new(n3));
}
