use std::cmp::max;

#[derive(Debug)]
struct Node {
    left_node: Option<Box<Node>>,
    value: i32,
    right_node: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            left_node: None,
            value: value,
            right_node: None,
        }
    }
}

impl Node {
    fn add(&mut self, value: i32) {
        if self.value == value {
            return;
        }
        let target_node = if value < self.value {
            &mut self.left_node
        } else {
            &mut self.right_node
        };
        match target_node {
            &mut Some(ref mut subnode) => subnode.add(value),
            &mut None => {
                let new_node = Node::new(value);
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }

    fn search(&self, value: i32) -> Option<&Node> {
        if self.value == value {
            return Some(self);
        } else {
            let target_node = if value < self.value {
                &self.left_node
            } else {
                &self.right_node
            };
            match target_node {
                Some(subnode) => subnode.search(value),
                None => None,
            }
        }
    }

    fn find_depth(&self) -> usize {
        let serach_node = (&self.left_node, &self.right_node);
        match serach_node {
            (Some(l), Some(r)) => max(l.find_depth(), r.find_depth()) + 1,
            (None, None) => 1,
            (Some(l), None) => l.find_depth() + 1,
            (None, Some(r)) => r.find_depth() + 1,
        }
    }
}

fn main() {
    let mut x = Node::new(2);
    let data = [
        16, 4, 2, 2, 11, 9, 0, 14, 11, 11, 9, 12, 7, 2, 12, 3, 9, 6, 12,
    ];
    for i in data.iter() {
        x.add(*i);
    }
    let depth = x.find_depth();
    assert_eq!(x.search(9).unwrap().value, 9);
    assert_eq!(x.search(90).is_none(), true);
    assert_eq!(depth, 7);
}
