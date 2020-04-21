// https://leetcode.com/problems/add-one-row-to-tree/
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
// Memory Usage: 2.7 MB
pub fn add_one_row(
    root: Option<Rc<RefCell<TreeNode>>>,
    v: i32,
    d: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn insert(root: Option<&mut Rc<RefCell<TreeNode>>>, value: i32, depth: i32, n: i32) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            if depth == n - 1 {
                let mut t = node.left.take();
                node.left = Some(Rc::new(RefCell::new(TreeNode::new(value))));
                node.left.as_mut().unwrap().borrow_mut().left = t;
                t = node.right.take();
                node.right = Some(Rc::new(RefCell::new(TreeNode::new(value))));
                node.right.as_mut().unwrap().borrow_mut().right = t;
            }
            {
                insert(node.left.as_mut(), value, depth + 1, n);
                insert(node.right.as_mut(), value, depth + 1, n);
            }
        }
    }

    if d == 1 {
        let mut n = TreeNode::new(v);
        n.left = root;
        return Some(Rc::new(RefCell::new(n)));
    }
    let mut root = root;
    insert(root.as_mut(), v, 1, d);
    root
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
