// https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves/
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
use std::{cell::RefCell, cmp::Ordering, rc::Rc};
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn helper(root: Option<&Rc<RefCell<TreeNode>>>) -> (usize, Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let left = helper(node.borrow().left.as_ref());
            let right = helper(node.borrow().right.as_ref());
            match left.0.cmp(&right.0) {
                Ordering::Equal => (left.0 + 1, Some(node.clone())),
                Ordering::Less => (right.0 + 1, right.1),
                Ordering::Greater => (left.0 + 1, left.1),
            }
        } else {
            (0, None)
        }
    }

    helper(root.as_ref()).1
}
// tree depth_first_search
#[test]
fn test1_1123() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let res1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let res2 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let t3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let res3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
    })));
    assert_eq!(lca_deepest_leaves(t1), res1);
    assert_eq!(lca_deepest_leaves(t2), res2);
    assert_eq!(lca_deepest_leaves(t3), res3);
}
