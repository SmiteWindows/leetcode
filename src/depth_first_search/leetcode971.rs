// https://leetcode.com/problems/flip-binary-tree-to-match-preorder-traversal/
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
pub fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
    fn walk(
        root: Option<&Rc<RefCell<TreeNode>>>,
        voyage: &[i32],
        index: &mut usize,
        flipped: &mut Vec<i32>,
    ) {
        if let Some(node) = root {
            let node = node.borrow();
            if node.val != voyage[*index] {
                flipped.clear();
                flipped.push(-1);
                return;
            }
            *index += 1;
            if *index < voyage.len()
                && node.left.is_some()
                && node.left.as_ref().unwrap().borrow().val != voyage[*index]
            {
                flipped.push(node.val);
                walk(node.right.as_ref(), voyage, index, flipped);
                walk(node.left.as_ref(), voyage, index, flipped);
            } else {
                walk(node.left.as_ref(), voyage, index, flipped);
                walk(node.right.as_ref(), voyage, index, flipped);
            }
        }
    }

    let mut flipped = Vec::new();
    let mut index = 0;
    walk(root.as_ref(), &voyage, &mut index, &mut flipped);
    if !flipped.is_empty() && flipped[0] == -1 {
        flipped.clear();
        flipped.push(-1);
    }
    flipped
}
// tree depth_first_search
#[test]
fn test2_971() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: None,
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let t3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    assert_eq!(flip_match_voyage(t1, vec![2, 1]), vec![-1]);
    assert_eq!(flip_match_voyage(t2, vec![1, 3, 2]), vec![1]);
    assert_eq!(flip_match_voyage(t3, vec![1, 2, 3]), vec![]);
}
