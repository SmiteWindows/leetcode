// https://leetcode.com/problems/construct-binary-search-tree-from-preorder-traversal/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::{cell::RefCell, rc::Rc};
pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut inorder = preorder.clone();
    inorder.sort_unstable();
    from_vec(&preorder, &inorder)
}

fn from_vec(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    let n = preorder.len();
    if n == 0 {
        None
    } else if n == 1 {
        Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))))
    } else {
        let i = inorder.binary_search(&preorder[0]).unwrap();
        Some(Rc::new(RefCell::new(TreeNode {
            val: preorder[0],
            left: from_vec(&preorder[1..=i], &inorder[0..i]),
            right: from_vec(&preorder[i + 1..], &inorder[i + 1..]),
        })))
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
fn test1_1008() {
    let res = Some(Rc::new(RefCell::new(TreeNode {
        val: 8,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(12)))),
        }))),
    })));
    assert_eq!(bst_from_preorder(vec![8, 5, 1, 7, 10, 12]), res);
}
