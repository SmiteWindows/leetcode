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
fn test2_1302() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
            }))),
        }))),
    })));
    assert_eq!(deepest_leaves_sum(root), 15);
}
