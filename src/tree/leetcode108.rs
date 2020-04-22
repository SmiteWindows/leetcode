// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/
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
// Memory Usage: 2.8 MB
pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn helper(left: i32, right: i32, nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if left > right {
            return None;
        }
        let p = (left + right) as usize / 2;
        let root = Some(Rc::new(RefCell::new(TreeNode::new(nums[p]))));
        root.as_ref()?.borrow_mut().left = helper(left, p as i32 - 1, nums);
        root.as_ref()?.borrow_mut().right = helper(p as i32 + 1, right, nums);
        root
    }

    let n = nums.len() as i32 - 1;
    helper(0, n, nums.as_slice())
}
// tree depth_first_search
#[test]
fn test1_108() {
    let nums = vec![-10, -3, 0, 5, 9];
    let res = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: -10,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(-3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        }))),
    })));
    assert_eq!(res, sorted_array_to_bst(nums));
}
