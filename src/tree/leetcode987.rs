// https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/
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
use std::{
    cell::RefCell,
    collections::{BTreeMap, BinaryHeap},
    rc::Rc,
};
pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut nodes = BTreeMap::new();
    walk(root.as_ref(), 0, 0, &mut nodes);
    vertical_order(nodes)
}

fn walk(
    root: Option<&Rc<RefCell<TreeNode>>>,
    x: i32,
    y: i32,
    nodes: &mut BTreeMap<i32, BTreeMap<i32, BinaryHeap<i32>>>,
) {
    if let Some(node) = root {
        let node = node.borrow();
        nodes
            .entry(x)
            .or_default()
            .entry(y)
            .or_default()
            .push(node.val);
        walk(node.left.as_ref(), x - 1, y - 1, nodes);
        walk(node.right.as_ref(), x + 1, y - 1, nodes);
    }
}

fn vertical_order(mut nodes: BTreeMap<i32, BTreeMap<i32, BinaryHeap<i32>>>) -> Vec<Vec<i32>> {
    let n = nodes.len();
    let mut res = vec![vec![]; n];
    for (i, col) in nodes.values_mut().enumerate() {
        for row in col.values_mut() {
            while let Some(smallest) = row.pop() {
                res[i].push(smallest);
            }
        }
    }
    res
}
// tree hash_table
#[test]
#[ignore]
fn test1_987() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    let res1 = vec![vec![9], vec![3, 15], vec![20], vec![7]];
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    let res2 = vec![vec![4], vec![2], vec![1, 5, 6], vec![3], vec![7]];
    assert_eq!(vertical_traversal(t1), res1);
    assert_eq!(vertical_traversal(t2), res2);
}
