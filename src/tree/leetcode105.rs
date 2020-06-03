// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
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
// Runtime: 0 ms
// Memory Usage: 2.7 MB
use std::{cell::RefCell, rc::Rc};
pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    helper(&preorder, &inorder)
}

fn helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() {
        return None;
    }
    let len = preorder.len();
    let root = Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))));
    let i = inorder
        .iter()
        .position(|&x| x == preorder[0])
        .expect("exist");
    root.as_deref()?.borrow_mut().left = helper(&preorder[1..=i], &inorder[0..i]);
    root.as_deref()?.borrow_mut().right = helper(&preorder[i + 1..len], &inorder[i + 1..len]);
    root
}
// tree depth_first_search array
#[test]
fn test1_105() {
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
