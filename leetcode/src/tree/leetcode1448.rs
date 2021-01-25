// https://leetcode-cn.com/problems/count-good-nodes-in-binary-tree/
// Runtime: 20 ms
// Memory Usage: 6.8 MB
use std::{cell::RefCell, rc::Rc};
pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    preorder(root.as_deref(), i32::MIN, &mut res);
    res
}

fn preorder(root: Option<&RefCell<TreeNode>>, max: i32, count: &mut i32) {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        if val >= max {
            *count += 1;
        }
        preorder(node.left.as_deref(), max.max(val), count);
        preorder(node.right.as_deref(), max.max(val), count);
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
fn test1_1448() {
    use leetcode_prelude::btree;
    assert_eq!(good_nodes(btree![3, 1, 4, 3, null, 1, 5]), 4);

    assert_eq!(good_nodes(btree![3, 3, null, 4, 2]), 3);

    assert_eq!(good_nodes(btree![1]), 1);
}
