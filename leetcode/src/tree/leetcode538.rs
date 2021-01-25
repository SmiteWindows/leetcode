// https://leetcode-cn.com/problems/convert-bst-to-greater-tree/
// Runtime: 4 ms
// Memory Usage: 2.9 MB
use std::{cell::RefCell, rc::Rc};
pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut sum = 0;
    walk(root.as_deref(), &mut sum);
    root
}

fn walk(root: Option<&RefCell<TreeNode>>, sum: &mut i32) {
    if let Some(node) = root {
        walk(node.borrow().right.as_deref(), sum);
        *sum += node.borrow().val;
        node.borrow_mut().val = *sum;
        walk(node.borrow().left.as_deref(), sum);
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
fn test1_538() {
    use leetcode_prelude::btree;
    assert_eq!(convert_bst(btree![5, 2, 13]), btree![18, 20, 13]);
}
