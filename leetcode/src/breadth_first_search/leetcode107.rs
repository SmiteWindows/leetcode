// https://leetcode-cn.com/problems/binary-tree-level-order-traversal-ii/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, collections::VecDeque, rc::Rc};
pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    if root.is_none() {
        return res;
    }
    let mut queue = VecDeque::new();
    queue.push_back(root);
    while !queue.is_empty() {
        let mut tmp = Vec::new();
        let len = queue.len();
        for i in 0..len {
            let node = queue.pop_front().unwrap().unwrap();
            let node = node.borrow();
            tmp.push(node.val);
            if node.left.is_some() {
                queue.push_back(node.left.clone());
            }
            if node.right.is_some() {
                queue.push_back(node.right.clone());
            }
        }
        res.insert(0, tmp);
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
// tree breadth_first_search
#[test]
fn test2_107() {
    use leetcode_prelude::{btree, vec2};
    assert_eq!(
        level_order_bottom(btree![3, 9, 20, null, null, 15, 7]),
        vec2![[15, 7], [9, 20], [3]]
    );
}
