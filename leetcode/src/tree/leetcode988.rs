// https://leetcode-cn.com/problems/smallest-string-starting-from-leaf/
// Runtime: 0 ms
// Memory Usage: 2.7 MB
use std::{cell::RefCell, rc::Rc};
pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let mut cur = Vec::new();
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
            if min.is_empty() || s < *min {
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
fn test1_988() {
    use leetcode_prelude::btree;
    assert_eq!(
        smallest_from_leaf(btree![0, 1, 2, 3, 4, 3, 4]),
        "dba".to_string()
    );

    assert_eq!(
        smallest_from_leaf(btree![25, 1, 3, 1, 3, 0, 2]),
        "adz".to_string()
    );

    assert_eq!(
        smallest_from_leaf(btree![2, 2, 1, null, 1, 0, null, 0]),
        "abc".to_string()
    );
}
