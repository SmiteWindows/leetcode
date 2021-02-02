// https://leetcode-cn.com/problems/flip-binary-tree-to-match-preorder-traversal/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, rc::Rc};
pub fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
    let mut nodes = Vec::new();
    let mut size = 0;
    if preorder(root.as_deref(), &mut size, &mut nodes, &voyage) {
        nodes
    } else {
        vec![-1]
    }
}

fn preorder(
    root: Option<&RefCell<TreeNode>>,
    size: &mut usize,
    nodes: &mut Vec<i32>,
    voyage: &[i32],
) -> bool {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        if voyage[*size] != val {
            return false;
        }
        *size += 1;
        if node.left.is_none() && node.right.is_none() {
            return true;
        }
        if node.left.is_some() {
            if node.left.as_deref().unwrap().borrow().val == voyage[*size] {
                preorder(node.left.as_deref(), size, nodes, voyage)
                    && preorder(node.right.as_deref(), size, nodes, voyage)
            } else {
                nodes.push(val);
                preorder(node.right.as_deref(), size, nodes, voyage)
                    && preorder(node.left.as_deref(), size, nodes, voyage)
            }
        } else {
            preorder(node.right.as_deref(), size, nodes, voyage)
        }
    } else {
        true
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
fn test1_971() {
    use leetcode_prelude::btree;
    assert_eq!(flip_match_voyage(btree![1, 2], vec![2, 1]), vec![-1]);

    assert_eq!(flip_match_voyage(btree![1, 2, 3], vec![1, 3, 2]), vec![1]);

    assert_eq!(flip_match_voyage(btree![1, 2, 3], vec![1, 2, 3]), vec![]);
}
