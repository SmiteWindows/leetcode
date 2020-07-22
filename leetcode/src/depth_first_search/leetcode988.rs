// https://leetcode.com/problems/smallest-string-starting-from-leaf/
// Runtime: 0 ms
// Memory Usage: 2.7 MB
use std::{cell::RefCell, rc::Rc};
pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let mut cur = vec![];
    let mut res = "".to_string();
    preorder(root.as_deref(), &mut cur, &mut res);
    res
}

fn preorder(root: Option<&RefCell<TreeNode>>, cur: &mut Vec<char>, min: &mut String) {
    if let Some(node) = root {
        let node = node.borrow();
        let val = (node.val as u8 + b'a') as char;
        cur.push(val);
        if node.left.is_none() && node.right.is_none() {
            let s = cur.iter().rev().copied().collect::<String>();
            if min == "" || s < *min {
                *min = s;
            }
        }
        preorder(node.left.as_deref(), cur, min);
        preorder(node.right.as_deref(), cur, min);
        cur.pop();
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
fn test2_988() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
    })));
    let s1 = String::from("dba");
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 25,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        }))),
    })));
    let s2 = String::from("adz");
    let t3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: None,
        }))),
    })));
    let s3 = String::from("abc");
    assert_eq!(smallest_from_leaf(t1), s1);
    assert_eq!(smallest_from_leaf(t2), s2);
    assert_eq!(smallest_from_leaf(t3), s3);
}
