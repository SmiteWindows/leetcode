// https://leetcode.com/problems/delete-nodes-and-return-forest/
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
// Runtime: 4 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, collections::HashSet, rc::Rc};
pub fn del_nodes(
    root: Option<Rc<RefCell<TreeNode>>>,
    to_delete: Vec<i32>,
) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let mut delete_set = HashSet::with_capacity(to_delete.len());
    let mut res = Vec::new();
    for delete in to_delete {
        delete_set.insert(delete);
    }
    helper(root.as_ref(), true, &mut delete_set, &mut res);
    res
}

fn helper(
    root: Option<&Rc<RefCell<TreeNode>>>,
    is_add: bool,
    delete_set: &mut HashSet<i32>,
    res: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
) -> bool {
    if let Some(node) = root {
        if delete_set.contains(&node.borrow().val) {
            if helper(node.borrow().left.as_ref(), true, delete_set, res) {
                node.borrow_mut().left = None;
            }
            if helper(node.borrow().right.as_ref(), true, delete_set, res) {
                node.borrow_mut().right = None;
            }
            return true;
        }
        if is_add {
            res.push(Some(node.clone()));
        }
        if helper(node.borrow().left.as_ref(), false, delete_set, res) {
            node.borrow_mut().left = None;
        }
        if helper(node.borrow().right.as_ref(), false, delete_set, res) {
            node.borrow_mut().right = None;
        }
    }
    false
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
