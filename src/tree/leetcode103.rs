// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/
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
use std::{cell::RefCell, rc::Rc};
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    fn helper(root: Option<&Rc<RefCell<TreeNode>>>, level: usize, res: &mut Vec<Vec<i32>>) {
        if let Some(node) = root {
            if level >= res.len() {
                let mut new_level = Vec::new();
                new_level.push(node.borrow().val);
                res.push(new_level);
            } else {
                if level%2==0 {
                    res[level].push(node.borrow().val);
                }else{
                    res[level].insert(0,node.borrow().val);
                }
            }
            if node.borrow().left.as_ref().is_some() {
                helper(node.borrow().left.as_ref(), level + 1, res);
            }
            if node.borrow().right.as_ref().is_some() {
                helper(node.borrow().right.as_ref(), level + 1, res);
            }
        }
    }
    let mut res = Vec::new();
    helper(root.as_ref(), 0, &mut res);
    res
}
// tree breadth_first_search stack
#[test]
fn test1_103() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    assert_eq!(
        vec![vec![3], vec![20, 9], vec![15, 7]],
        zigzag_level_order(root)
    );
}
