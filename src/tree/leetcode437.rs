// https://leetcode.com/problems/path-sum-iii/
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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
use std::{cell::RefCell, collections::HashMap, rc::Rc};
// Runtime: 24 ms
// Memory Usage: 2.2 MB
pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
    fn walk(
        root: Option<&Rc<RefCell<TreeNode>>>,
        target: i32,
        prefix_sum_count: &mut HashMap<i32, i32>,
        mut curr_sum: i32,
    ) -> i32 {
        if let Some(node) = root {
            let mut res = 0;
            curr_sum += node.borrow().val;
            res += *prefix_sum_count.entry(curr_sum - target).or_default();
            let v = *prefix_sum_count.entry(curr_sum).or_default();
            prefix_sum_count.insert(curr_sum, v + 1);
            res += walk(
                node.borrow().right.as_ref(),
                target,
                prefix_sum_count,
                curr_sum,
            );
            res += walk(
                node.borrow().left.as_ref(),
                target,
                prefix_sum_count,
                curr_sum,
            );
            prefix_sum_count.insert(curr_sum, prefix_sum_count.get(&curr_sum).unwrap() - 1);
            res
        } else {
            0
        }
    }
    let mut prefix_sum_count = HashMap::new();
    prefix_sum_count.insert(0, 1);
    walk(root.as_ref(), sum, &mut prefix_sum_count, 0)
}
// tree
#[test]
fn test1_437() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 10,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(-2)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: -3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(11)))),
        }))),
    })));
    assert_eq!(3, path_sum(root, 8));
}
