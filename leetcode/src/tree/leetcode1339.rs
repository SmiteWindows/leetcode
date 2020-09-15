// https://leetcode-cn.com/problems/maximum-product-of-splitted-binary-tree/
// Runtime: 20 ms
// Memory Usage: 10.2 MB
use std::{cell::RefCell, rc::Rc};
pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let sum = postorder_sum(root.as_deref());
    let mut res = 0;
    postorder_product(root.as_deref(), &mut res, sum);
    (res % 1_000_000_007) as i32
}

fn postorder_sum(root: Option<&RefCell<TreeNode>>) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        let left = postorder_sum(node.left.as_deref());
        let right = postorder_sum(node.right.as_deref());
        val + left + right
    } else {
        0
    }
}

fn postorder_product(root: Option<&RefCell<TreeNode>>, max: &mut i64, sum: i32) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        let left = postorder_product(node.left.as_deref(), max, sum);
        let right = postorder_product(node.right.as_deref(), max, sum);
        let res = val + left + right;
        *max = (*max).max((sum - res) as i64 * res as i64);
        res
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
// tree depth_first_search dynamic_programming
#[test]
fn test1_1339() {
    use leetcode_prelude::btree;
    assert_eq!(max_product(btree![1, 2, 3, 4, 5, 6]), 110);

    assert_eq!(max_product(btree![1, null, 2, 3, 4, null, null, 5, 6]), 90);

    assert_eq!(max_product(btree![2, 3, 9, 10, 7, 8, 6, 5, 4, 11, 1]), 1025);

    assert_eq!(max_product(btree![1, 1]), 1);
}
