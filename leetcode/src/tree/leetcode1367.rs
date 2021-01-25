// https://leetcode-cn.com/problems/linked-list-in-binary-tree/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
use std::{cell::RefCell, rc::Rc};
pub fn is_sub_path(mut head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut path = vec![];
    while let Some(node) = head {
        path.push(node.val);
        head = node.next;
    }
    let mut cur = vec![];
    let mut res = false;
    preorder(root.as_deref(), &mut cur, &mut res, &path);
    res
}

fn preorder(root: Option<&RefCell<TreeNode>>, cur: &mut Vec<i32>, found: &mut bool, path: &[i32]) {
    if let Some(node) = root {
        let node = node.borrow();
        cur.push(node.val);
        if cur.ends_with(path) {
            *found = true;
        }
        preorder(node.left.as_deref(), cur, found, path);
        preorder(node.right.as_deref(), cur, found, path);
        cur.pop();
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<Self>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self { next: None, val }
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
// tree linked_list dynamic_programming
#[test]
fn test1_1367() {
    use leetcode_prelude::{btree, list};
    assert_eq!(
        is_sub_path(
            list![4, 2, 8],
            btree![1, 4, 4, null, 2, 2, null, 1, null, 6, 8, null, null, null, null, 1, 3]
        ),
        true
    );

    assert_eq!(
        is_sub_path(
            list![1, 4, 2, 6],
            btree![1, 4, 4, null, 2, 2, null, 1, null, 6, 8, null, null, null, null, 1, 3]
        ),
        true
    );

    assert_eq!(
        is_sub_path(
            list![1, 4, 2, 6, 8],
            btree![1, 4, 4, null, 2, 2, null, 1, null, 6, 8, null, null, null, null, 1, 3]
        ),
        false
    );
}
