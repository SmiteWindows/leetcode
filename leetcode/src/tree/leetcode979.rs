// https://leetcode-cn.com/problems/distribute-coins-in-binary-tree/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::{cell::RefCell, rc::Rc};
pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    postorder(root.as_deref(), &mut res);
    res
}

fn postorder(root: Option<&RefCell<TreeNode>>, sum: &mut i32) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        let left = postorder(node.left.as_deref(), sum);
        let right = postorder(node.right.as_deref(), sum);
        let m = val + left + right - 1;
        *sum += m.abs();
        m
    } else {
        0
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
fn test1_979() {
    use leetcode_prelude::btree;
    assert_eq!(distribute_coins(btree![3, 0, 0]), 2);

    assert_eq!(distribute_coins(btree![0, 3, 0]), 3);

    assert_eq!(distribute_coins(btree![1, 0, 2]), 2);

    assert_eq!(distribute_coins(btree![1, 0, 0, null, 3]), 4);
}
