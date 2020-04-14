// https://leetcode.com/problems/binary-tree-paths/
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
pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    fn helper(root: Option<&Rc<RefCell<TreeNode>>>, mut path: String, paths: &mut Vec<String>) {
        if let Some(node) = root {
            path += &(node.borrow().val).to_string();
            if node.borrow().left.as_ref().is_none() && node.borrow().right.as_ref().is_none() {
                paths.push(path);
            } else {
                path += "->";
                helper(node.borrow().left.as_ref(), path.clone(), paths);
                helper(node.borrow().right.as_ref(), path, paths);
            }
        }
    }
    let mut paths: Vec<String> = Vec::new();
    helper(root.as_ref(), "".to_string(), &mut paths);
    paths
}
// tree depth_first_search
#[test]
fn test1_257() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let res = vec![String::from("1->2->5"), String::from("1->3")];
    assert_eq!(res, binary_tree_paths(root));
}
