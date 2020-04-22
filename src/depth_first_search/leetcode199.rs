// https://leetcode.com/problems/binary-tree-right-side-view/
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
// Memory Usage: 2.1 MB
pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn walk(root: Option<&Rc<RefCell<TreeNode>>>, level: usize, res: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            let val = node.val;
            if res.len() <= level {
                res.push(val);
            } else {
                res[level] = val;
            }
            walk(node.left.as_ref(), level + 1, res);
            walk(node.right.as_ref(), level + 1, res);
        }
    }
    
    let mut res = Vec::new();
    walk(root.as_ref(), 0, &mut res);
    res
}
// tree depth_first_search breadth_first_search
#[test]
fn test3_199() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
    })));
    let res = vec![1, 3, 4];
    assert_eq!(res, right_side_view(root));
}
