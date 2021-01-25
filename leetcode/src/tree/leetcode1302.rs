// https://leetcode-cn.com/problems/deepest-leaves-sum/
// Runtime: 4 ms
// Memory Usage: 3 MB
use std::{
    cell::RefCell,
    cmp::Ordering::{Equal, Greater, Less},
    rc::Rc,
};
pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max = 0;
    let mut res = 0;
    preorder_dfs(root.as_deref(), 0, &mut max, &mut res);
    res
}

fn preorder_dfs(root: Option<&RefCell<TreeNode>>, level: usize, max: &mut usize, sum: &mut i32) {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        match level.cmp(max) {
            Equal => *sum += val,
            Greater => {
                *max = level;
                *sum = val;
            }
            Less => {}
        }
        preorder_dfs(node.left.as_deref(), level + 1, max, sum);
        preorder_dfs(node.right.as_deref(), level + 1, max, sum);
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
fn test1_1302() {
    use leetcode_prelude::btree;
    assert_eq!(
        deepest_leaves_sum(btree![1, 2, 3, 4, 5, null, 6, 7, null, null, null, null, 8]),
        15
    );
}
