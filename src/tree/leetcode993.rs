// https://leetcode.com/problems/cousins-in-binary-tree/
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
use std::{cell::RefCell, collections::HashMap, rc::Rc};
pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
    let mut depth = HashMap::new();
    let mut parent = HashMap::new();
    walk(root.as_ref(), None, &mut depth, &mut parent);
    depth.get(&x) == depth.get(&y) && parent.get(&x) != parent.get(&y)
}

fn walk(
    root: Option<&Rc<RefCell<TreeNode>>>,
    par: Option<Rc<RefCell<TreeNode>>>,
    depth: &mut HashMap<i32, i32>,
    parent: &mut HashMap<i32, Option<Rc<RefCell<TreeNode>>>>,
) {
    if let Some(node) = root {
        if let Some(n) = par.as_ref() {
            let val = n.borrow().val;
            depth.insert(node.borrow().val, 1 + depth.get(&val).expect("exist"));
        } else {
            depth.insert(node.borrow().val, 0);
        }
        parent.insert(node.borrow().val, par);
        walk(
            node.borrow().left.as_ref(),
            Some(node.clone()),
            depth,
            parent,
        );
        walk(
            node.borrow().right.as_ref(),
            Some(node.clone()),
            depth,
            parent,
        );
    }
}
// tree breadth_first_search
#[test]
fn test1_993() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
    })));
    let t3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    assert_eq!(is_cousins(t1, 4, 3), false);
    assert_eq!(is_cousins(t2, 5, 4), true);
    assert_eq!(is_cousins(t3, 2, 3), false);
}
