// https://leetcode.com/problems/trim-a-binary-search-tree/
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
use std::{cell::RefCell, rc::Rc};
// Runtime: 0 ms
// Memory Usage: 3.1 MB
pub fn trim_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    l: i32,
    r: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn helper(
        root: Option<&Rc<RefCell<TreeNode>>>,
        l: i32,
        r: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let left = helper(node.borrow().left.as_ref(), l, r);
            let right = helper(node.borrow().right.clone().as_ref(), l, r);
            let val = node.borrow().val;
            if val > r {
                return left;
            } else if val < l {
                return right;
            } else {
                node.borrow_mut().left = left;
                node.borrow_mut().right = right;
            }
            Some(node.clone())
        } else {
            None
        }
    }
    helper(root.as_ref(), l, r)
}
// tree
#[test]
fn test1_669() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    let res1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    assert_eq!(trim_bst(t1, 1, 2), res1);
    let res2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: None,
        }))),
        right: None,
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
    })));
    assert_eq!(trim_bst(t2, 1, 3), res2);
}
