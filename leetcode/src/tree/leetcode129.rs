// https://leetcode-cn.com/problems/sum-root-to-leaf-numbers/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
use std::{cell::RefCell, rc::Rc};
pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    preorder(root.as_deref(), 0, &mut res);
    res
}

fn preorder(root: Option<&RefCell<TreeNode>>, path: i32, sum: &mut i32) {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        let path = path * 10 + val;
        if node.left.is_none() && node.right.is_none() {
            *sum += path;
        }
        preorder(node.left.as_deref(), path, sum);
        preorder(node.right.as_deref(), path, sum);
    }
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
// tree depth_first_search
#[test]
fn test1_129() {
    let root1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    assert_eq!(25, sum_numbers(root1));
    let root2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
    })));
    assert_eq!(1026, sum_numbers(root2));
}
