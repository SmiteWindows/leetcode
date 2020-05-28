// https://leetcode.com/problems/distribute-coins-in-binary-tree/
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
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::{cell::RefCell, rc::Rc};
pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    walk(root.as_deref(), &mut res);
    res
}

fn walk(root: Option<&RefCell<TreeNode>>, res: &mut i32) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let left = walk(node.left.as_deref(), res);
        let right = walk(node.right.as_deref(), res);
        *res += left.abs() + right.abs();
        node.val + left + right - 1
    } else {
        0
    }
}
// tree depth_first_search
#[test]
fn test2_979() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
    })));
    let t3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    let t4 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
    })));
    assert_eq!(distribute_coins(t1), 2);
    assert_eq!(distribute_coins(t2), 3);
    assert_eq!(distribute_coins(t3), 2);
    assert_eq!(distribute_coins(t4), 4);
}
