// https://leetcode-cn.com/problems/unique-binary-search-trees-ii/
// Runtime: 0 ms
// Memory Usage: 2.6 MB
use std::{cell::RefCell, rc::Rc};
pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if n == 0 {
        Vec::new()
    } else {
        helper(1, n)
    }
}

fn helper(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let mut res = Vec::new();
    if start > end {
        res.push(None);
        return res;
    }
    for i in start..=end {
        let left_trees = helper(start, i - 1);
        let right_trees = helper(i + 1, end);
        for lt in left_trees.iter() {
            for rt in right_trees.iter() {
                let curr_tree = Some(Rc::new(RefCell::new(TreeNode::new(i))));
                curr_tree.as_deref().unwrap().borrow_mut().left = lt.clone();
                curr_tree.as_deref().unwrap().borrow_mut().right = rt.clone();
                res.push(curr_tree);
            }
        }
    }
    res
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
// tree dynamic_programming
#[test]
fn test1_95() {
    use leetcode_prelude::vec_btree;
    assert_eq!(
        generate_trees(3),
        vec_btree![
            [1, null, 2, null, 3],
            [1, null, 3, 2],
            [2, 1, 3],
            [3, 1, null, null, 2],
            [3, 2, null, 1]
        ]
    );
}
