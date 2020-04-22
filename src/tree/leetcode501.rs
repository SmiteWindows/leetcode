// https://leetcode.com/problems/find-mode-in-binary-search-tree/
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
use std::{cell::RefCell, cmp::Ordering, rc::Rc};
// Runtime: 0 ms
// Memory Usage: 2.9 MB
pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn walk(
        root: Option<&Rc<RefCell<TreeNode>>>,
        max: &mut i32,
        cur: &mut i32,
        count: &mut i32,
        res: &mut Vec<i32>,
    ) {
        if let Some(node) = root {
            let node = node.borrow();
            walk(node.left.as_ref(), max, cur, count, res);
            if node.val != *cur {
                *count = 0;
            }
            *count += 1;
            match (*max).cmp(&*count) {
                Ordering::Equal => res.push(node.val),
                Ordering::Less => {
                    *max = *count;
                    res.clear();
                    res.push(node.val);
                }
                Ordering::Greater => {}
            }
            *cur = node.val;
            walk(node.right.as_ref(), max, cur, count, res);
        }
    }
    
    let (mut max, mut cur, mut count) = (0, 0, 0);
    let mut res = Vec::new();
    walk(root.as_ref(), &mut max, &mut cur, &mut count, &mut res);
    res
}
// tree
#[test]
fn test1_501() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: None,
        }))),
    })));
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    assert_eq!(vec![2], find_mode(t1));
    assert_eq!(vec![1], find_mode(t2));
}
