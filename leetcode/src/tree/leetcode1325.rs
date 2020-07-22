// https://leetcode.com/problems/delete-leaves-with-a-given-value/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
use std::{cell::RefCell, rc::Rc};
pub fn remove_leaf_nodes(
    root: Option<Rc<RefCell<TreeNode>>>,
    target: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    postorder(root, target)
}

fn postorder(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        let val = node.borrow().val;
        let left = node.borrow_mut().left.take();
        let right = node.borrow_mut().right.take();
        let left = postorder(left, target);
        let right = postorder(right, target);
        if left.is_none() && right.is_none() && val == target {
            None
        } else {
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        }
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
fn test1_1325() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
    }))); // [1,2,3,2,null,2,4]
    let res1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
    }))); // [1,null,3,null,4]
    assert_eq!(remove_leaf_nodes(t1, 2), res1);
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    }))); // [1,3,3,3,2]
    let res2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        }))),
        right: None,
    }))); // [1,3,null,null,2]
    assert_eq!(remove_leaf_nodes(t2, 3), res2);
    let t3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: None,
            }))),
            right: None,
        }))),
        right: None,
    }))); // [1,2,null,2,null,2]
    let res3 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    assert_eq!(remove_leaf_nodes(t3, 2), res3);
    let t4 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
    }))); // [1,1,1]
    let res4 = None;
    assert_eq!(remove_leaf_nodes(t4, 1), res4);
    let t5 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    }))); // [1,2,3]
    let res5 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    }))); // [1,2,3]
    assert_eq!(remove_leaf_nodes(t5, 1), res5);
}
