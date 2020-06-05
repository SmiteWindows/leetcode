// https://leetcode.com/problems/binary-tree-paths/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, rc::Rc};
pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    let mut paths = Vec::new();
    helper(root.as_deref(), "".to_string(), &mut paths);
    paths
}

fn helper(root: Option<&RefCell<TreeNode>>, mut path: String, paths: &mut Vec<String>) {
    if let Some(node) = root {
        let node = node.borrow();
        path += &(node.val).to_string();
        if node.left.is_none() && node.right.is_none() {
            paths.push(path);
        } else {
            path += "->";
            helper(node.left.as_deref(), path.clone(), paths);
            helper(node.right.as_deref(), path, paths);
        }
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
// tree depth_first_search
#[test]
fn test2_257() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let res = vec!["1->2->5", "1->3"];
    assert_eq!(res, binary_tree_paths(root));
}
