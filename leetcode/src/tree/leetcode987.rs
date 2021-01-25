// https://leetcode-cn.com/problems/vertical-order-traversal-of-a-binary-tree/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
    rc::Rc,
};
pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut nodes = BTreeMap::new();
    preorder(root.as_deref(), 0, 0, &mut nodes);
    vertical_order(nodes)
}

fn preorder(
    root: Option<&RefCell<TreeNode>>,
    x: i32,
    y: i32,
    nodes: &mut BTreeMap<i32, BTreeMap<i32, BinaryHeap<Reverse<i32>>>>,
) {
    if let Some(node) = root {
        let node = node.borrow();
        nodes
            .entry(x)
            .or_default()
            .entry(y)
            .or_default()
            .push(Reverse(node.val));
        preorder(node.left.as_deref(), x - 1, y + 1, nodes);
        preorder(node.right.as_deref(), x + 1, y + 1, nodes);
    }
}

fn vertical_order(
    mut nodes: BTreeMap<i32, BTreeMap<i32, BinaryHeap<Reverse<i32>>>>,
) -> Vec<Vec<i32>> {
    let n = nodes.len();
    let mut res = vec![vec![]; n];
    for (i, col) in nodes.values_mut().enumerate() {
        for row in col.values_mut() {
            while let Some(Reverse(smallest)) = row.pop() {
                res[i].push(smallest);
            }
        }
    }
    res
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<Self>>>,
    pub right: Option<Rc<RefCell<Self>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}
// tree hash_table
#[test]
fn test1_987() {
    use leetcode_prelude::{btree, vec2};
    assert_eq!(
        vertical_traversal(btree![3, 9, 20, null, null, 15, 7]),
        vec2![[9], [3, 15], [20], [7]]
    );

    assert_eq!(
        vertical_traversal(btree![1, 2, 3, 4, 5, 6, 7]),
        vec2![[4], [2], [1, 5, 6], [3], [7]]
    );
}
