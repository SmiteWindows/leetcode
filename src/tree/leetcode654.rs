// https://leetcode.com/problems/maximum-binary-tree/
// Runtime: 8 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, rc::Rc};
pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    construct(&nums, 0, nums.len())
}

fn get_max(nums: &[i32], l: usize, r: usize) -> usize {
    let mut max_i = l;
    for i in l..r {
        if nums[max_i] < nums[i] {
            max_i = i;
        }
    }
    max_i
}

fn construct(nums: &[i32], l: usize, r: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if l == r {
        return None;
    }
    let max_i = get_max(nums, l, r);
    let mut node = TreeNode::new(nums[max_i]);
    node.left = construct(nums, l, max_i);
    node.right = construct(nums, max_i + 1, r);
    Some(Rc::new(RefCell::new(node)))
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
fn test1_654() {
    let nums = vec![3, 2, 1, 6, 0, 5];
    let res = Some(Rc::new(RefCell::new(TreeNode {
        val: 6,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: None,
        }))),
    })));
    assert_eq!(res, construct_maximum_binary_tree(nums));
}
