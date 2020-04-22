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
use std::{cell::RefCell, cmp::max, rc::Rc};
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
    fn get_height(root: Option<&Rc<RefCell<TreeNode>>>) -> usize {
        if let Some(node) = root {
            let node = node.borrow();
            1 + max(
                get_height(node.left.as_ref()),
                get_height(node.right.as_ref()),
            )
        } else {
            0
        }
    }

    fn fill(
        res: &mut Vec<Vec<String>>,
        root: Option<&Rc<RefCell<TreeNode>>>,
        i: usize,
        l: usize,
        r: usize,
    ) {
        if let Some(node) = root {
            let node = node.borrow();
            res[i][(l + r) / 2] = node.val.to_string();
            fill(res, node.left.as_ref(), i + 1, l, (l + r) / 2);
            fill(res, node.right.as_ref(), i + 1, (l + r + 1) / 2, r);
        }
    }

    let m = get_height(root.as_ref());
    let n = (1 << m) - 1;
    let mut res = vec![vec![String::from(""); n]; m];
    let len = res[0].len();
    fill(&mut res, root.as_ref(), 0, 0, len);
    res
}
// tree
#[test]
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
