// https://leetcode.com/problems/insert-into-a-binary-search-tree/
// Runtime: 8 ms
// Memory Usage: 2.5 MB
use std::{cell::RefCell, rc::Rc};
pub fn insert_into_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    postorder_insert(root, val)
}

fn postorder_insert(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        let node_val = node.borrow().val;
        let left = node.borrow_mut().left.take();
        let right = node.borrow_mut().right.take();
        if val > node_val {
            Some(Rc::new(RefCell::new(TreeNode {
                val: node_val,
                left,
                right: postorder_insert(right, val),
            })))
        } else {
            Some(Rc::new(RefCell::new(TreeNode {
                val: node_val,
                left: postorder_insert(left, val),
                right,
            })))
        }
    } else {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
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
fn test1_701() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
    })));
    let res1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: None,
        }))),
    })));
    assert_eq!(res1, insert_into_bst(t1, 5));
    // let t2 = Some(Rc::new(RefCell::new(TreeNode {
    //     val: 4,
    //     left: Some(Rc::new(RefCell::new(TreeNode {
    //         val: 2,
    //         left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
    //         right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    //     }))),
    //     right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
    // })));
    // let res2 = Some(Rc::new(RefCell::new(TreeNode {
    //     val: 5,
    //     left: Some(Rc::new(RefCell::new(TreeNode {
    //         val: 2,
    //         left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
    //         right: Some(Rc::new(RefCell::new(TreeNode {
    //             val: 3,
    //             left: None,
    //             right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
    //         }))),
    //     }))),
    //     right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
    // })));
    // assert_eq!(res2, insert_into_bst(t2, 5));
}
