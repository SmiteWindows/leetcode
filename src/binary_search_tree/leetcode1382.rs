// https://leetcode.com/problems/balance-a-binary-search-tree/
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
// Runtime: 16 ms
// Memory Usage: 3.3 MB
use std::{cell::RefCell, rc::Rc};
pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut sort_list = Vec::new();
    inorder(root.as_ref(), &mut sort_list);
    build_tree(&sort_list)
}

fn inorder(root: Option<&Rc<RefCell<TreeNode>>>, sort_list: &mut Vec<i32>) {
    if let Some(node) = root {
        let node = node.borrow();
        inorder(node.left.as_ref(), sort_list);
        sort_list.push(node.val);
        inorder(node.right.as_ref(), sort_list);
    }
}

fn build_tree(sort_list: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    let n = sort_list.len();
    if n == 0 {
        return None;
    }
    let m = n / 2;
    let mut node = TreeNode::new(sort_list[m]);
    node.left = build_tree(&sort_list[0..m]);
    node.right = build_tree(&sort_list[m + 1..n]);
    Some(Rc::new(RefCell::new(node)))
}
// binary_search_tree
#[test]
fn test1_1382() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
        }))),
    })));
    let res = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
    })));
    assert_eq!(res, balance_bst(root));
}
