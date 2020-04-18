// https://leetcode.com/problems/binary-tree-inorder-traversal/
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
// Memory Usage: 2.1 MB
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn helper(root: Option<&Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(node) = root {
            if node.borrow().left.as_ref().is_some() {
                helper(node.borrow().left.as_ref(), res);
            }
            res.push(node.borrow().val);
            if node.borrow().right.as_ref().is_some() {
                helper(node.borrow().right.as_ref(), res);
            }
        }
    }
    let mut res = Vec::new();
    helper(root.as_ref(), &mut res);
    res
}
// tree hash_table stack
#[test]
fn test1_94() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: None,
        }))),
    })));
    assert_eq!(vec![1, 3, 2], inorder_traversal(root));
}
