// https://leetcode.com/problems/most-frequent-subtree-sum/
// Runtime: 0 ms
// Memory Usage: 3 MB
use std::{cell::RefCell, collections::HashMap, rc::Rc};
pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut max_value = 0;
    let mut res = Vec::new();
    walk(root.as_ref(), &mut map, &mut max_value);
    for i in map.keys() {
        if map.get(i).expect("exist") == &max_value {
            res.push(*i);
        }
    }
    res
}

fn walk(
    root: Option<&Rc<RefCell<TreeNode>>>,
    map: &mut HashMap<i32, i32>,
    max_value: &mut i32,
) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let left = walk(node.left.as_ref(), map, max_value);
        let right = walk(node.right.as_ref(), map, max_value);
        let sum = node.val + left + right;
        *map.entry(sum).or_default() += 1;
        *max_value = (*max_value).max(*map.get(&sum).expect("exist"));
        sum
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
// tree hash_table
#[test]
fn test2_508() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(-3)))),
    })));
    let mut s1 = find_frequent_tree_sum(t1);
    s1.sort();
    let res = vec![-3, 2, 4];
    assert_eq!(res, s1);
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(-5)))),
    })));
    let res = vec![2];
    assert_eq!(res, find_frequent_tree_sum(t2));
}
