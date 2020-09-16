// https://leetcode-cn.com/problems/binary-tree-level-order-traversal/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, collections::VecDeque, rc::Rc};
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    if let Some(node) = root {
        let mut queue = VecDeque::new();
        queue.push_back(Some(node));
        let mut level = 0;
        while !queue.is_empty() {
            res.push(Vec::new());
            let level_length = queue.len();
            for i in 0..level_length {
                let n = queue.pop_front().unwrap().unwrap();
                let n = n.borrow();
                res[level].push(n.val);
                if n.left.is_some() {
                    queue.push_back(n.left.clone());
                }
                if n.right.is_some() {
                    queue.push_back(n.right.clone());
                }
            }
            level += 1;
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
// tree breadth_first_search
#[test]
fn test2_102() {
    use leetcode_prelude::{btree, vec2};
    assert_eq!(
        level_order(btree![3, 9, 20, null, null, 15, 7]),
        vec2![[3], [9, 20], [15, 7]]
    );
}
