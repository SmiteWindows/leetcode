// https://leetcode.com/problems/binary-search-tree-iterator/
// Runtime: 12 ms
// Memory Usage: 9.4 MB
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
struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut obj = Self { stack: Vec::new() };
        obj.push_all_left(root);
        obj
    }

    /** @return the next smallest number */
    fn next(&mut self) -> i32 {
        let curr = self.stack.pop().unwrap();
        let val = curr.borrow().val;
        self.push_all_left(curr.borrow().right.as_ref().map(Rc::clone));
        val
    }

    /** @return whether we have a next smallest number */
    fn has_next(&self) -> bool {
        self.stack.last().is_some()
    }

    fn push_all_left(&mut self, mut root: Option<Rc<RefCell<TreeNode>>>) {
        while let Some(node) = root {
            self.stack.push(Rc::clone(&node));
            root = node.borrow().left.as_ref().map(Rc::clone);
        }
    }
}
/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
// design stack tree
#[test]
fn test3_173() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 7,
        left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 15,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(20)))),
        }))),
    })));
    let mut it = BSTIterator::new(root);
    assert_eq!(it.next(), 3);
    assert_eq!(it.next(), 7);
    assert_eq!(it.has_next(), true);
    assert_eq!(it.next(), 9);
    assert_eq!(it.has_next(), true);
    assert_eq!(it.next(), 15);
    assert_eq!(it.has_next(), true);
    assert_eq!(it.next(), 20);
    assert_eq!(it.has_next(), false);
}
