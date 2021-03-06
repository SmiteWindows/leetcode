// https://leetcode-cn.com/problems/binary-tree-zigzag-level-order-traversal/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::{cell::RefCell, collections::VecDeque, rc::Rc};
pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    if let Some(node) = root {
        let mut queue = VecDeque::new();
        queue.push_back(Some(node));
        queue.push_back(None);
        let mut level_list = Vec::new();
        let mut is_order_left = true;
        while !queue.is_empty() {
            if let Some(curr) = queue.pop_front().unwrap() {
                let curr = curr.borrow();
                if is_order_left {
                    level_list.push(curr.val);
                } else {
                    level_list.insert(0, curr.val);
                }
                if curr.left.is_some() {
                    queue.push_back(curr.left.clone());
                }
                if curr.right.is_some() {
                    queue.push_back(curr.right.clone());
                }
            } else {
                res.push(level_list);
                level_list = Vec::new();
                if !queue.is_empty() {
                    queue.push_back(None);
                }
                is_order_left = !is_order_left;
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
// tree breadth_first_search stack
#[test]
fn test2_103() {
    use leetcode_prelude::{btree, vec2};
    assert_eq!(
        zigzag_level_order(btree![3, 9, 20, null, null, 15, 7]),
        vec2![[3], [20, 9], [15, 7]]
    );
}
