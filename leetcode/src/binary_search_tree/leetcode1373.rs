// https://leetcode.com/problems/maximum-sum-bst-in-binary-tree/
// Runtime: 28 ms
// Memory Usage: 10.7 MB
use std::{cell::RefCell, rc::Rc};
pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_sum = 0;
    walk(root.as_deref(), &mut max_sum);
    max_sum
}

fn walk(root: Option<&RefCell<TreeNode>>, max_sum: &mut i32) -> (bool, i32, i32, i32) {
    if let Some(node) = root {
        let node = node.borrow();
        let left = walk(node.left.as_deref(), max_sum);
        let right = walk(node.right.as_deref(), max_sum);
        let mut sum = 0;
        if !left.0 || !right.0 || node.val >= right.1 || node.val <= left.2 {
            return (false, 0, 0, 0);
        }
        let cur_min = if node.left.is_some() {
            left.1
        } else {
            node.val
        };
        let cur_max = if node.right.is_some() {
            right.2
        } else {
            node.val
        };
        sum += node.val + left.3 + right.3;
        *max_sum = sum.max(*max_sum);
        (true, cur_min, cur_max, sum)
    } else {
        (true, i32::MAX, i32::MIN, 0)
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
// binary_search_tree dynamic_programming
#[test]
fn test1_1373() {
    use leetcode_prelude::btree;
    assert_eq!(
        max_sum_bst(btree![
            1, 4, 3, 2, 4, 2, 5, null, null, null, null, null, null, 4, 6
        ]),
        20
    );
    assert_eq!(max_sum_bst(btree![4, 3, null, 1, 2]), 2);
    assert_eq!(max_sum_bst(btree![-4, -2, -5]), 0);
    assert_eq!(max_sum_bst(btree![2, 1, 3]), 6);
    assert_eq!(max_sum_bst(btree![5, 4, 8, 3, null, 6, 3]), 7);
}
