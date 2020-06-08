// https://leetcode.com/problems/delete-nodes-and-return-forest/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, collections::HashSet, iter::FromIterator, rc::Rc};
pub fn del_nodes(
    root: Option<Rc<RefCell<TreeNode>>>,
    to_delete: Vec<i32>,
) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let mut res = vec![];
    let nodes = HashSet::from_iter(to_delete);
    let (root, forest) = postorder(root, &nodes);
    if root.is_some() {
        res.push(root);
    }
    for t in forest {
        res.push(t);
    }
    res
}

fn postorder(
    root: Option<Rc<RefCell<TreeNode>>>,
    nodes: &HashSet<i32>,
) -> (
    Option<Rc<RefCell<TreeNode>>>,
    Vec<Option<Rc<RefCell<TreeNode>>>>,
) {
    if let Some(node) = root {
        let val = node.borrow_mut().val;
        let left = node.borrow_mut().left.take();
        let right = node.borrow_mut().right.take();
        let (left_root, left_forest) = postorder(left, nodes);
        let (right_root, right_forest) = postorder(right, nodes);
        let mut forest = vec![];
        for t in left_forest {
            forest.push(t);
        }
        for t in right_forest {
            forest.push(t);
        }
        if nodes.contains(&val) {
            if left_root.is_some() {
                forest.push(left_root);
            }
            if right_root.is_some() {
                forest.push(right_root);
            }
            (None, forest)
        } else {
            node.borrow_mut().left = left_root;
            node.borrow_mut().right = right_root;
            (Some(node), forest)
        }
    } else {
        (None, vec![])
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
fn test2_1110() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    let s1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: None,
        }))),
        right: None,
    })));
    let s2 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    let s3 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    assert_eq!(del_nodes(root, vec![3, 5]), vec![s1, s2, s3]);
}
