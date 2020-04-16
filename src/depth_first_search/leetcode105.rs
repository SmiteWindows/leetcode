// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
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
use std::{cell::RefCell, collections::HashMap, rc::Rc};
// Runtime: 4 ms
// Memory Usage: 2.6 MB
pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() {
            return None;
        }
        let len = inorder.len();
        let root = Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))));
        let mid = inorder.iter().position(|&x| x == preorder[0]).unwrap();
        root.as_ref()?.borrow_mut().left = helper(&preorder[1..=mid], &inorder[0..mid]);
        root.as_ref()?.borrow_mut().right = helper(&preorder[mid + 1..len], &inorder[mid + 1..len]);
        root
    }
    helper(&preorder, &inorder)
}
// tree depth_first_search array
#[test]
fn test3_105() {
    let res = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    assert_eq!(
        res,
        build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7])
    );
}
