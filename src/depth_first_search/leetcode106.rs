// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
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
// Memory Usage: 2.7 MB
pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn helper(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() {
            return None;
        }
        let len = inorder.len();
        let root = Some(Rc::new(RefCell::new(TreeNode::new(postorder[len-1]))));
        let i = inorder.iter().position(|&x| x == postorder[len-1]).unwrap();
        root.as_ref()?.borrow_mut().left = helper(&inorder[0..i], &postorder[0..i]);
        root.as_ref()?.borrow_mut().right = helper(&inorder[i + 1..len], &postorder[i..len-1]);
        root
    }

    helper(&inorder, &postorder)
}
// tree depth_first_search array
#[test]
fn test3_106() {
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
        build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3])
    );
}
