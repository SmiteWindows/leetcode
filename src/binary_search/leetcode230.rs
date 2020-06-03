// https://leetcode.com/problems/kth-smallest-element-in-a-bst/
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
// Runtime: 0 ms
// Memory Usage: 3.1 MB
use std::{cell::RefCell, cmp::Ordering, rc::Rc};
pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    helper(root.as_deref(), k)
}

fn count(root: Option<&RefCell<TreeNode>>) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        1 + count(node.left.as_deref()) + count(node.right.as_deref())
    } else {
        0
    }
}

fn helper(root: Option<&RefCell<TreeNode>>, k: i32) -> i32 {
    let node = root.expect("exist").borrow();
    let n = count(node.left.as_deref());
    match (n + 1).cmp(&k) {
        Ordering::Equal => node.val,
        Ordering::Less => helper(node.right.as_deref(), k - n - 1),
        Ordering::Greater => helper(node.left.as_deref(), k),
    }
}
// tree binary_search
#[test]
fn test2_230() {
    let root1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
    })));
    assert_eq!(1, kth_smallest(root1, 1));
    let root2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
    })));
    assert_eq!(3, kth_smallest(root2, 3));
}
