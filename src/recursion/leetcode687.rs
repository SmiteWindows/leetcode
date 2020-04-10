// https://leetcode.com/problems/longest-univalue-path/
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
use std::{cell::RefCell, rc::Rc, cmp::max};
// Runtime: 16 ms
// Memory Usage: 3 MB
pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn arrow_length(root: Option<&Rc<RefCell<TreeNode>>>, res: i32) -> (i32, i32) {
        if let Some(node) = root {
            let (left, res) = arrow_length(node.as_ref().borrow().left.as_ref(), res);
            let (right, res) = arrow_length(node.as_ref().borrow().right.as_ref(), res);
            let (mut arrow_left, mut arrow_right) = (0, 0);
            if let Some(left_node) = node.as_ref().borrow().left.as_ref() {
                if left_node.as_ref().borrow().val == node.as_ref().borrow().val {
                    arrow_left += left + 1;
                }
            }
            if let Some(right_node) = node.as_ref().borrow().right.as_ref() {
                if right_node.as_ref().borrow().val == node.as_ref().borrow().val {
                    arrow_right += right + 1;
                }
            }
            (
                max(arrow_left, arrow_right),
                max(res, arrow_left + arrow_right),
            )
        } else {
            (0, res)
        }
    }
    arrow_length(root.as_ref(), 0).1
}
// tree recursion
#[test]
fn test2_687() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
    })));
    assert_eq!(2, longest_univalue_path(t1));
    assert_eq!(2, longest_univalue_path(t2));
}
