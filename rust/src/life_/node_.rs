struct Node {
    next: Option<Box<Node>>,
}

#[test]
fn test_box_() {
    let mut node1 = Node { next: None };
    let mut node2 = Node { next: None };
    node1.next = Some(Box::new(node2));
}
