// https://leetcode-cn.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, rc::Rc};
pub fn construct_from_pre_post(pre: Vec<i32>, post: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    Some(build(&mut 0, &mut 0, &pre, &post))
}

fn build(i: &mut usize, j: &mut usize, pre: &[i32], post: &[i32]) -> Rc<RefCell<TreeNode>> {
    let val = pre[*i];
    *i += 1;
    let mut left = None;
    let mut right = None;
    if val != post[*j] {
        left = Some(build(i, j, pre, post));
    }
    if val != post[*j] {
        right = Some(build(i, j, pre, post));
    }
    *j += 1;
    Rc::new(RefCell::new(TreeNode { val, left, right }))
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
fn test1_889() {
    use leetcode_prelude::btree;
    assert_eq!(
        construct_from_pre_post(vec![1, 2, 4, 5, 3, 6, 7], vec![4, 5, 2, 6, 7, 3, 1]),
        btree![1, 2, 3, 4, 5, 6, 7]
    );
}
