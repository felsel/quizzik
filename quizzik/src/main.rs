fn main() {
    println!("Hello, world!");

    let a = Node {
        name: "joel",
        adjacent: vec![Some(Node {
            name: "joelly",
            adjacent: vec![None],
        })],
    };

    dbg!(a);
}

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    adjacent: Vec<Option<Node<'a>>>,
}
