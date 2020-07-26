// https://leetcode.com/problems/binary-search-tree-to-greater-sum-tree/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, rc::Rc};
pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    inorder(root.as_deref(), &mut 0);
    root
}

fn inorder(root: Option<&RefCell<TreeNode>>, sum: &mut i32) {
    if let Some(node) = root {
        inorder(node.borrow().right.as_deref(), sum);
        *sum += node.borrow().val;
        node.borrow_mut().val = *sum;
        inorder(node.borrow().left.as_deref(), sum);
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
// binary_search_tree
#[test]
fn test1_1038() {
    use leetcode_prelude::btree;
    assert_eq!(
        bst_to_gst(btree![
            4, 1, 6, 0, 2, 5, 7, null, null, null, 3, null, null, null, 8
        ]),
        btree![30, 36, 21, 36, 35, 26, 15, null, null, null, 33, null, null, null, 8]
    );
    // let root = Some(Rc::new(RefCell::new(TreeNode {
    //     val: 4,
    //     left: Some(Rc::new(RefCell::new(TreeNode {
    //         val: 1,
    //         left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
    //         right: Some(Rc::new(RefCell::new(TreeNode {
    //             val: 2,
    //             left: None,
    //             right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    //         }))),
    //     }))),
    //     right: Some(Rc::new(RefCell::new(TreeNode {
    //         val: 6,
    //         left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
    //         right: Some(Rc::new(RefCell::new(TreeNode {
    //             val: 7,
    //             left: None,
    //             right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
    //         }))),
    //     }))),
    // })));
    // let res = Some(Rc::new(RefCell::new(TreeNode {
    //     val: 30,
    //     left: Some(Rc::new(RefCell::new(TreeNode {
    //         val: 36,
    //         left: Some(Rc::new(RefCell::new(TreeNode::new(36)))),
    //         right: Some(Rc::new(RefCell::new(TreeNode {
    //             val: 35,
    //             left: None,
    //             right: Some(Rc::new(RefCell::new(TreeNode::new(33)))),
    //         }))),
    //     }))),
    //     right: Some(Rc::new(RefCell::new(TreeNode {
    //         val: 21,
    //         left: Some(Rc::new(RefCell::new(TreeNode::new(26)))),
    //         right: Some(Rc::new(RefCell::new(TreeNode {
    //             val: 15,
    //             left: None,
    //             right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
    //         }))),
    //     }))),
    // })));
    // assert_eq!(res, bst_to_gst(root));
}
