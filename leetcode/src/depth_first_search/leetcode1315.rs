// https://leetcode-cn.com/problems/sum-of-nodes-with-even-valued-grandparent/
// Runtime: 4 ms
// Memory Usage: 3 MB
use std::{cell::RefCell, rc::Rc};
pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    preorder(root.as_deref(), false, false, &mut res);
    res
}

fn preorder(root: Option<&RefCell<TreeNode>>, parent: bool, grandparent: bool, sum: &mut i32) {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        if grandparent {
            *sum += val;
        }
        preorder(node.left.as_deref(), val % 2 == 0, parent, sum);
        preorder(node.right.as_deref(), val % 2 == 0, parent, sum);
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
fn test2_1315() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 6,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
        }))),
    })));
    assert_eq!(sum_even_grandparent(root), 18);
}
