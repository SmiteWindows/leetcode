// https://leetcode-cn.com/problems/symmetric-tree/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::{cell::RefCell, rc::Rc};
pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut queue = vec![root.clone(), root];
    while !queue.is_empty() {
        let t1 = queue.pop().unwrap();
        let t2 = queue.pop().unwrap();
        if t1.is_none() && t2.is_none() {
            continue;
        }
        if t1.is_none() || t2.is_none() {
            return false;
        }
        let (t1, t2) = (t1.unwrap(), t2.unwrap());
        let (t1, t2) = (t1.borrow(), t2.borrow());
        if t1.val != t2.val {
            return false;
        }
        queue.push(t1.left.clone());
        queue.push(t2.right.clone());
        queue.push(t1.right.clone());
        queue.push(t2.left.clone());
    }
    true
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
// tree depth_first_search breadth_first_search
#[test]
fn test2_101() {
    use leetcode_prelude::btree;
    assert_eq!(is_symmetric(btree![1, 2, 2, 3, 4, 4, 3]), true);
    assert_eq!(is_symmetric(btree![1, 2, 2, null, 3, null, 3]), false);
}
