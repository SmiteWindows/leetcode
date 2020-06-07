// https://leetcode.com/problems/count-good-nodes-in-binary-tree/
// Runtime: 20 ms
// Memory Usage: 6.8 MB
use std::{cell::RefCell, rc::Rc};
pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    preorder(root.as_deref(), i32::MIN, &mut res);
    res
}

fn preorder(root: Option<&RefCell<TreeNode>>, max: i32, count: &mut i32) {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        if val >= max {
            *count += 1;
        }
        preorder(node.left.as_deref(), max.max(val), count);
        preorder(node.right.as_deref(), max.max(val), count);
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
fn test1_1448() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        }))),
        right: None,
    })));
    let t3 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    assert_eq!(good_nodes(t1), 4);
    assert_eq!(good_nodes(t2), 3);
    assert_eq!(good_nodes(t3), 1);
}
