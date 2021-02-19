// https://leetcode-cn.com/problems/add-one-row-to-tree/
// Runtime: 0 ms
// Memory Usage: 2.7 MB
use std::{cell::RefCell, rc::Rc};
pub fn add_one_row(
    root: Option<Rc<RefCell<TreeNode>>>,
    v: i32,
    d: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    if d == 1 {
        let mut n = TreeNode::new(v);
        n.left = root;
        return Some(Rc::new(RefCell::new(n)));
    }
    postorder(root.as_deref(), 1, v, d)
}

fn postorder(
    root: Option<&RefCell<TreeNode>>,
    depth: i32,
    v: i32,
    d: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        let mut node = node.borrow_mut();
        let val = node.val;
        let left = node.left.take();
        let right = node.right.take();
        let mut left = postorder(left.as_deref(), depth + 1, v, d);
        let mut right = postorder(right.as_deref(), depth + 1, v, d);
        if depth == d - 1 {
            left = Some(Rc::new(RefCell::new(TreeNode {
                val: v,
                left,
                right: None,
            })));
            right = Some(Rc::new(RefCell::new(TreeNode {
                val: v,
                left: None,
                right,
            })));
        }
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    } else {
        None
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
fn test1_623() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: None,
        }))),
    })));
    let res1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                right: None,
            }))),
        }))),
    })));
    assert_eq!(res1, add_one_row(t1, 1, 2));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        }))),
        right: None,
    })));
    let res2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
        }))),
        right: None,
    })));
    assert_eq!(res2, add_one_row(t2, 1, 3));
}
