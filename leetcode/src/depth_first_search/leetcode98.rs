// https://leetcode-cn.com/problems/validate-binary-search-tree/
// Runtime: 0 ms
// Memory Usage: 2.9 MB
use std::{cell::RefCell, rc::Rc};
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut prev = None;
    let mut res = true;
    inorder(root.as_deref(), &mut |x| {
        if let Some(y) = prev {
            if x <= y {
                res = false;
            }
        }
        prev = Some(x);
    });
    res
}

fn inorder(root: Option<&RefCell<TreeNode>>, visit: &mut dyn FnMut(i32)) {
    if let Some(node) = root {
        let node = node.borrow();
        inorder(node.left.as_deref(), visit);
        visit(node.val);
        inorder(node.right.as_deref(), visit);
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
fn test2_98() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
        }))),
    })));
    assert_eq!(true, is_valid_bst(t1));
    assert_eq!(false, is_valid_bst(t2));
}
