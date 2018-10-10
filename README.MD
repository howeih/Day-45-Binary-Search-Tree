Day 45: Binary Search Tree
<br>
BST is another very interesting data structure. It allows for fast lookup, insertion and removal in expected O(log n) time. What’s even more important, BST keeps the data sorted and supports interval retrieval in time O(k+log n) for k items. That makes binary trees a key tool in RDBMS.
<br>
There are many improvements to a basic binary tree, most profound would be 2–3 tree, Red-Black tree, B-tree, B+tree, B\*tree. But don’t get confused when learning them for the first [or second, or third] time.
<br>
They all are just basic BSTs with additional constraints, tweaks and optimizations. When you think of them in terms of basic binary trees, the life is often much easier.
<br>
2–3 tree is a perfectly balanced BST [almost, there’s a hidden constant]
<br>
LLRB-tree is BST implementation of 2–3 tree with path coloring constraint
<br>
RB-tree is BST implementation of 2–3–4 tree with path coloring constraint
<br>
B-tree is a perfectly balanced BST with subtree block alignment
<br>
Besides the usual implementation, today I’d like to focus on an interesting property: When adding uniformly distributed data into unbalanced BST, the height of the tree is O(log n).
<br>
run:

```
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
```
