// https://leetcode.com/problems/all-elements-in-two-binary-search-trees/
// Runtime: 32 ms
// Memory Usage: 3 MB
use std::{cell::RefCell, rc::Rc};
pub fn get_all_elements(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> Vec<i32> {
    let mut res = vec![];
    inorder(root1.as_deref(), &mut res);
    inorder(root2.as_deref(), &mut res);
    res.sort();
    res
}

fn inorder(root: Option<&RefCell<TreeNode>>, v: &mut Vec<i32>) {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        inorder(node.left.as_deref(), v);
        v.push(val);
        inorder(node.right.as_deref(), v);
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
// tree sort
#[test]
fn test2_1305() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let t3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode::new(-10)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(10)))),
    })));
    let t4 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
    })));
    let t5 = None;
    let t6 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
    })));
    let t7 = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode::new(-10)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(10)))),
    })));
    let t8 = None;
    let t9 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
    })));
    let t10 = Some(Rc::new(RefCell::new(TreeNode {
        val: 8,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: None,
    })));
    assert_eq!(get_all_elements(t1, t2), vec![0, 1, 1, 2, 3, 4]);
    assert_eq!(get_all_elements(t3, t4), vec![-10, 0, 0, 1, 2, 5, 7, 10]);
    assert_eq!(get_all_elements(t5, t6), vec![0, 1, 2, 5, 7]);
    assert_eq!(get_all_elements(t7, t8), vec![-10, 0, 10]);
    assert_eq!(get_all_elements(t9, t10), vec![1, 1, 8, 8]);
}
