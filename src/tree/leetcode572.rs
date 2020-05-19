// https://leetcode.com/problems/subtree-of-another-tree/
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
// Runtime: 4 ms
// Memory Usage: 2 MB
use std::{cell::RefCell, rc::Rc};
pub fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn is_same(s: Option<&Rc<RefCell<TreeNode>>>, t: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        if s.is_none() && t.is_none() {
            return true;
        }
        if s.is_none() || t.is_none() {
            return false;
        }
        let (sn, tn) = (s.unwrap().borrow(), t.unwrap().borrow());
        if sn.val != tn.val {
            false
        } else {
            is_same(sn.left.as_ref(), tn.left.as_ref())
                && is_same(sn.right.as_ref(), tn.right.as_ref())
        }
    }

    fn helper(s: Option<&Rc<RefCell<TreeNode>>>, t: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        if s.is_none() {
            return false;
        }
        if is_same(s, t) {
            true
        } else {
            let sn = s.unwrap().borrow();
            helper(sn.left.as_ref(), t) || helper(sn.right.as_ref(), t)
        }
    }

    helper(s.as_ref(), t.as_ref())
}
// tree
#[test]
fn test1_572() {
    let s1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
    })));
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    assert_eq!(true, is_subtree(s1, t1));
    let s2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    assert_eq!(false, is_subtree(s2, t2));
}
