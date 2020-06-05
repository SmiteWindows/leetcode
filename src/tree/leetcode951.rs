// https://leetcode.com/problems/flip-equivalent-binary-trees/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, rc::Rc};
pub fn flip_equiv(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    helper(root1.as_deref(), root2.as_deref())
}

fn helper(root1: Option<&RefCell<TreeNode>>, root2: Option<&RefCell<TreeNode>>) -> bool {
    if root1 == root2 {
        return true;
    }
    if root1.is_none() || root2.is_none() {
        return false;
    }
    let node1 = root1.expect("exist").borrow();
    let node2 = root2.expect("exist").borrow();
    if node1.val != node2.val {
        return false;
    }
    helper(node1.left.as_deref(), node2.left.as_deref())
        && helper(node1.right.as_deref(), node2.right.as_deref())
        || helper(node1.left.as_deref(), node2.right.as_deref())
            && helper(node1.right.as_deref(), node2.left.as_deref())
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
// tree
#[test]
fn test1_951() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: None,
        }))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        }))),
    })));
    assert_eq!(flip_equiv(t1, t2), true);
}
