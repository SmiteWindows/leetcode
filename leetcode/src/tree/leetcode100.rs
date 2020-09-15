// https://leetcode-cn.com/problems/same-tree/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::{cell::RefCell, rc::Rc};
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_same(p.as_deref(), q.as_deref())
}

fn is_same(p: Option<&RefCell<TreeNode>>, q: Option<&RefCell<TreeNode>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (None, _) | (_, None) => false,
        (Some(pr), Some(qr)) => {
            let (pr, qr) = (pr.borrow(), qr.borrow());
            if pr.val == qr.val {
                is_same(pr.left.as_deref(), qr.left.as_deref())
                    && is_same(pr.right.as_deref(), qr.right.as_deref())
            } else {
                false
            }
        }
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
fn test1_100() {
    use leetcode_prelude::btree;
    assert_eq!(is_same_tree(btree![1, 2, 3], btree![1, 2, 3]), true);
    assert_eq!(is_same_tree(btree![1, 2], btree![1, null, 2]), false);
    assert_eq!(is_same_tree(btree![1, 2, 1], btree![1, 1, 2]), false);
}
