// https://leetcode.com/problems/balanced-binary-tree/
// Runtime: 0 ms
// Memory Usage: 2.6 MB
use std::{cell::RefCell, rc::Rc};
pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    helper(root.as_deref())
}

fn height(root: Option<&RefCell<TreeNode>>) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        1 + height(node.left.as_deref()).max(height(node.right.as_deref()))
    } else {
        -1
    }
}

fn helper(root: Option<&RefCell<TreeNode>>) -> bool {
    if let Some(node) = root {
        let node = node.borrow();
        (height(node.left.as_deref()) - height(node.right.as_deref())).abs() < 2
            && helper(node.left.as_deref())
            && helper(node.right.as_deref())
    } else {
        true
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
fn test1_110() {
    let root1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    assert_eq!(true, is_balanced(root1));
    let root2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    assert_eq!(false, is_balanced(root2));
}
