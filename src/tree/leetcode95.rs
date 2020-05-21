// https://leetcode.com/problems/unique-binary-search-trees-ii/
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
        for lt in &left_trees {
            for rt in &right_trees {
                let curr_tree = Some(Rc::new(RefCell::new(TreeNode::new(i))));
                curr_tree.as_ref().unwrap().borrow_mut().left = lt.clone();
                curr_tree.as_ref().unwrap().borrow_mut().right = rt.clone();
                res.push(curr_tree);
            }
        }
    }
    res
}
// tree dynamic_programming
#[test]
fn test1_95() {
    let s1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
    })));
    let s2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: None,
        }))),
    })));
    let s3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let s4 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        }))),
        right: None,
    })));
    let s5 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: None,
        }))),
        right: None,
    })));
    assert_eq!(vec![s1, s2, s3, s4, s5], generate_trees(3));
}
