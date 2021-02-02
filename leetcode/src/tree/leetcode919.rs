// https://leetcode-cn.com/problems/complete-binary-tree-inserter/
// Runtime: 4 ms
// Memory Usage: 2.6 MB
use std::{cell::RefCell, rc::Rc};
struct CBTInserter {
    stack: Vec<Option<Rc<RefCell<TreeNode>>>>,
}

impl CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = Vec::new();
        stack.push(root);
        let mut i = 0;
        while i < stack.len() {
            let left = stack[i].as_deref().unwrap().borrow_mut().left.clone();
            let right = stack[i].as_deref().unwrap().borrow_mut().right.clone();
            if left.is_some() {
                stack.push(left);
            }
            if right.is_some() {
                stack.push(right);
            }
            i += 1;
        }
        Self { stack }
    }

    fn insert(&mut self, v: i32) -> i32 {
        let link = Some(Rc::new(RefCell::new(TreeNode::new(v))));
        let n = self.stack.len();
        self.stack.push(link.clone());
        let mut parent = self.stack[(n - 1) / 2].as_deref().unwrap().borrow_mut();
        if n % 2 == 1 {
            parent.left = link;
        } else {
            parent.right = link;
        }
        parent.val
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.stack[0].clone()
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
/**
 * Your CBTInserter object will be instantiated and called as such:
 * let obj = CBTInserter::new(root);
 * let ret_1: i32 = obj.insert(v);
 * let ret_2: Option<Rc<RefCell<TreeNode>>> = obj.get_root();
 */
// tree
#[test]
fn test1_919() {
    use leetcode_prelude::btree;
    let mut obj = CBTInserter::new(btree![1]);
    assert_eq!(obj.insert(2), 1);
    assert_eq!(obj.get_root(), btree![1, 2]);
    let mut obj = CBTInserter::new(btree![1, 2, 3, 4, 5, 6]);
    assert_eq!(obj.insert(7), 3);
    assert_eq!(obj.insert(8), 4);
    assert_eq!(obj.get_root(), btree![1, 2, 3, 4, 5, 6, 7, 8]);
}
