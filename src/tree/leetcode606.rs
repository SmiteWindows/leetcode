// https://leetcode.com/problems/construct-string-from-binary-tree/
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
// Memory Usage: 2.7 MB
use std::{cell::RefCell, rc::Rc};
pub fn tree2str(t: Option<Rc<RefCell<TreeNode>>>) -> String {
    helper(t.as_deref())
}

fn helper(t: Option<&RefCell<TreeNode>>) -> String {
    if let Some(n) = t {
        let n = n.borrow();
        if n.left.is_none() && n.right.is_none() {
            return n.val.to_string() + "";
        }
        if n.right.is_none() {
            return n.val.to_string() + "(" + &helper(n.left.as_deref()) + ")";
        }
        n.val.to_string() + "(" + &helper(n.left.as_deref()) + ")(" + &helper(n.right.as_deref()) + ")"
    } else {
        String::from("")
    }
}
// tree string
#[test]
fn test1_606() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let res1 = String::from("1(2(4))(3)");
    assert_eq!(res1, tree2str(t1));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let res2 = String::from("1(2()(4))(3)");
    assert_eq!(res2, tree2str(t2));
}
