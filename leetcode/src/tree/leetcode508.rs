// https://leetcode-cn.com/problems/most-frequent-subtree-sum/
// Runtime: 0 ms
// Memory Usage: 3 MB
use std::{cell::RefCell, collections::HashMap, rc::Rc};
pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut max_value = 0;
    let mut res = Vec::new();
    walk(root.as_deref(), &mut map, &mut max_value);
    for i in map.keys() {
        if map.get(i).unwrap() == &max_value {
            res.push(*i);
        }
    }
    res
}

fn walk(root: Option<&RefCell<TreeNode>>, map: &mut HashMap<i32, i32>, max_value: &mut i32) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let left = walk(node.left.as_deref(), map, max_value);
        let right = walk(node.right.as_deref(), map, max_value);
        let sum = node.val + left + right;
        *map.entry(sum).or_default() += 1;
        *max_value = (*max_value).max(*map.get(&sum).unwrap());
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
fn test1_508() {
    use leetcode_prelude::{assert_eq_sorted, btree};
    assert_eq_sorted!(find_frequent_tree_sum(btree![5, 2, -3]), vec![2, -3, 4]);
    assert_eq_sorted!(find_frequent_tree_sum(btree![5, 2, -5]), vec![2]);
}
