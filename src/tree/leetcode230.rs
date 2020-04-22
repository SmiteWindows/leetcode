// https://leetcode.com/problems/kth-smallest-element-in-a-bst/
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
// Memory Usage: 3.1 MB
pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    fn inorder(root: Option<&Rc<RefCell<TreeNode>>>, k: i32, num: &mut i32, res: &mut i32) {
        if let Some(node) = root {
            let node = node.borrow();
            inorder(node.left.as_ref(), k, num, res);
            *num += 1;
            if *num == k {
                *res = node.val;
                return;
            }
            inorder(node.right.as_ref(), k, num, res);
        }
    }
    
    let mut num = 0;
    let mut res = 0;
    inorder(root.as_ref(), k, &mut num, &mut res);
    res
}
// tree binary_search
#[test]
fn test1_230() {
    let root1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
    })));
    assert_eq!(1, kth_smallest(root1, 1));
    let root2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
    })));
    assert_eq!(3, kth_smallest(root2, 3));
}
