// https://leetcode.com/problems/print-binary-tree/
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
use std::{cell::RefCell, rc::Rc};

pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
    todo!()
}
// tree
#[test]
#[ignore]
fn test1_655() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: None,
    })));
    let res1 = vec![
        vec![String::from(""), String::from("1"), String::from("")],
        vec![String::from("2"), String::from(""), String::from("")],
    ];
    assert_eq!(print_tree(t1), res1);
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let res2 = vec![
        vec![
            String::from(""),
            String::from(""),
            String::from(""),
            String::from("1"),
            String::from(""),
            String::from(""),
            String::from(""),
        ],
        vec![
            String::from(""),
            String::from("2"),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from("3"),
            String::from(""),
        ],
        vec![
            String::from(""),
            String::from(""),
            String::from("4"),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
        ],
    ];
    assert_eq!(print_tree(t2), res2);
    let t3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: None,
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
    })));
    let res3 = vec![
        vec![
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from("1"),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
        ],
        vec![
            String::from(""),
            String::from(""),
            String::from(""),
            String::from("2"),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from("5"),
            String::from(""),
            String::from(""),
            String::from(""),
        ],
        vec![
            String::from(""),
            String::from("3"),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
        ],
        vec![
            String::from("4"),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
        ],
    ];
    assert_eq!(print_tree(t3), res3);
}
