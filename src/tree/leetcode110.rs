// https://leetcode.com/problems/balanced-binary-tree/
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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
use std::{cell::RefCell, cmp::max, rc::Rc};
// Runtime: 0 ms
// Memory Usage: 2.6 MB
pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn height(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            1 + max(
                height(node.borrow().left.as_ref()),
                height(node.borrow().right.as_ref()),
            )
        } else {
            -1
        }
    }
    fn helper(root: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            (height(node.borrow().left.as_ref()) - height(node.borrow().right.as_ref())).abs() < 2
                && helper(node.borrow().left.as_ref())
                && helper(node.borrow().right.as_ref())
        } else {
            true
        }
    }
    helper(root.as_ref())
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
