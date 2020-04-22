// https://leetcode.com/problems/balanced-binary-tree/
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
use std::{cell::RefCell, cmp::max, rc::Rc};
// Runtime: 0 ms
// Memory Usage: 2.7 MB
pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn helper(root: Option<&Rc<RefCell<TreeNode>>>) -> (i32, bool) {
        if let Some(node) = root {
            let node = node.borrow();
            let left = helper(node.left.as_ref());
            if !left.1 {
                return (0, false);
            }
            let right = helper(node.right.as_ref());
            if !right.1 {
                return (0, false);
            }
            (1 + max(left.0, right.0), ((left.0 - right.0).abs() < 2))
        } else {
            (-1, true)
        }
    }

    helper(root.as_ref()).1
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
