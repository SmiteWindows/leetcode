// https://leetcode-cn.com/problems/all-possible-full-binary-trees/
// Runtime: 16 ms
// Memory Usage: 5.2 MB
use std::{cell::RefCell, rc::Rc};
pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if n % 2 == 0 {
        return vec![];
    }
    if n == 1 {
        vec![Some(Rc::new(RefCell::new(TreeNode::new(0))))]
    } else {
        let mut res = vec![];
        let mut l = 1;
        let mut r = n - 1 - l;
        while r > 0 {
            let left_trees = all_possible_fbt(l);
            let right_trees = all_possible_fbt(r);
            for left in &left_trees {
                for right in &right_trees {
                    res.push(Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: left.clone(),
                        right: right.clone(),
                    }))));
                }
            }
            r -= 2;
            l += 2;
        }
        res
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
// tree recursion
#[test]
fn test1_894() {
    use leetcode_prelude::vec_btree;
    assert_eq!(
        all_possible_fbt(7),
        vec_btree![
            [0, 0, 0, null, null, 0, 0, null, null, 0, 0],
            [0, 0, 0, null, null, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, null, null, null, null, 0, 0],
            [0, 0, 0, 0, 0, null, null, 0, 0]
        ]
    );
}
