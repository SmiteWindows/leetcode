// https://leetcode.com/problems/maximum-width-of-binary-tree/
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
use std::{cell::RefCell, collections::HashMap, rc::Rc};
// Runtime: 0 ms
// Memory Usage: 2.4 MB
pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn walk(
        root: Option<&Rc<RefCell<TreeNode>>>,
        depth: i32,
        pos: i32,
        res: &mut i32,
        left_map: &mut HashMap<i32, i32>,
    ) {
        if let Some(node) = root {
            let node = node.borrow();
            left_map.entry(depth).or_insert(pos);
            *res = (pos - left_map.get(&depth).unwrap() + 1).max(*res);
            walk(node.left.as_ref(), depth + 1, pos * 2, res, left_map);
            walk(node.right.as_ref(), depth + 1, pos * 2 + 1, res, left_map);
        }
    }

    let mut res = 0;
    let mut left_map = HashMap::new();
    walk(root.as_ref(), 0, 0, &mut res, &mut left_map);
    res
}
// tree
#[test]
fn test1_662() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        }))),
    })));
    assert_eq!(width_of_binary_tree(t1), 4);
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: None,
    })));
    assert_eq!(width_of_binary_tree(t2), 2);
    let t3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    assert_eq!(width_of_binary_tree(t3), 2);
    let t4 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: None,
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        }))),
    })));
    assert_eq!(width_of_binary_tree(t4), 8);
}
