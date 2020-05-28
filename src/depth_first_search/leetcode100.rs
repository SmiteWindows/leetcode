// https://leetcode.com/problems/same-tree/
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
// tree depth_first_search
#[test]
fn test1_100() {
    let tree1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let tree2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    assert_eq!(true, is_same_tree(tree1, tree2));
    let tree3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: None,
    })));
    let tree4 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    assert_eq!(false, is_same_tree(tree3, tree4));
    let tree5 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
    })));
    let tree6 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    assert_eq!(false, is_same_tree(tree5, tree6));
}
