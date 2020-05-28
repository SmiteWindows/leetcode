// https://leetcode.com/problems/two-sum-iv-input-is-a-bst/
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
// Memory Usage: 3.1 MB
use std::{cell::RefCell, collections::HashSet, rc::Rc};
pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
    let mut set = HashSet::new();
    walk(root.as_deref(), k, &mut set)
}

fn walk(root: Option<&RefCell<TreeNode>>, k: i32, set: &mut HashSet<i32>) -> bool {
    if let Some(node) = root {
        let node = node.borrow();
        if set.contains(&(k - node.val)) {
            return true;
        }
        set.insert(node.val);
        walk(node.left.as_deref(), k, set) || walk(node.right.as_deref(), k, set)
    } else {
        false
    }
}
// tree
#[test]
fn test1_653() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    assert_eq!(true, find_target(t1, 9));
    assert_eq!(false, find_target(t2, 28));
}
