// https://leetcode-cn.com/problems/symmetric-tree/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, rc::Rc};
pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_mirror(root.as_deref(), root.as_deref())
}

fn is_mirror(p: Option<&RefCell<TreeNode>>, q: Option<&RefCell<TreeNode>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (None, _) | (_, None) => false,
        (Some(pr), Some(qr)) => {
            let (pr, qr) = (pr.borrow(), qr.borrow());
            (pr.val == qr.val)
                && is_mirror(pr.right.as_deref(), qr.left.as_deref())
                && is_mirror(pr.left.as_deref(), qr.right.as_deref())
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
// tree depth_first_search breadth_first_search
#[test]
fn test1_101() {
    use leetcode_prelude::btree;
    assert_eq!(is_symmetric(btree![1, 2, 2, 3, 4, 4, 3]), true);
    assert_eq!(is_symmetric(btree![1, 2, 2, null, 3, null, 3]), false);
}
