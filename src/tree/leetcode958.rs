// https://leetcode.com/problems/check-completeness-of-a-binary-tree/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::{cell::RefCell, collections::HashSet, rc::Rc};
pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut nodes = HashSet::new();
    let count = postorder(root.as_deref(), 1, &mut nodes);
    nodes.len() == count && nodes.into_iter().all(|x| x <= count as u32)
}

fn postorder(root: Option<&RefCell<TreeNode>>, id: u32, nodes: &mut HashSet<u32>) -> usize {
    if let Some(node) = root {
        let node = node.borrow();
        let left = postorder(node.left.as_deref(), id << 1, nodes);
        let right = postorder(node.right.as_deref(), (id << 1) | 1, nodes);
        nodes.insert(id);
        left + right + 1
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
// tree
#[test]
fn test1_958() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: None,
        }))),
    }))); // [1,2,3,4,5,6]
    assert_eq!(is_complete_tree(t1), true);
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    }))); // [1,2,3,4,5,null,7]
    assert_eq!(is_complete_tree(t2), false);
}
