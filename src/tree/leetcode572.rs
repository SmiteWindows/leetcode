// https://leetcode.com/problems/subtree-of-another-tree/
// Runtime: 4 ms
// Memory Usage: 2 MB
use std::{cell::RefCell, rc::Rc};
pub fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
    helper(s.as_deref(), t.as_deref())
}

fn is_same(s: Option<&RefCell<TreeNode>>, t: Option<&RefCell<TreeNode>>) -> bool {
    if s.is_none() && t.is_none() {
        return true;
    }
    if s.is_none() || t.is_none() {
        return false;
    }
    let (sn, tn) = (s.expect("exist").borrow(), t.expect("exist").borrow());
    if sn.val != tn.val {
        false
    } else {
        is_same(sn.left.as_deref(), tn.left.as_deref())
            && is_same(sn.right.as_deref(), tn.right.as_deref())
    }
}

fn helper(s: Option<&RefCell<TreeNode>>, t: Option<&RefCell<TreeNode>>) -> bool {
    if s.is_none() {
        return false;
    }
    if is_same(s, t) {
        true
    } else {
        let sn = s.expect("exist").borrow();
        helper(sn.left.as_deref(), t) || helper(sn.right.as_deref(), t)
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
