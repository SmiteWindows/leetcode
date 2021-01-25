// https://leetcode-cn.com/problems/find-elements-in-a-contaminated-binary-tree/
// Runtime: 4 ms
// Memory Usage: 4.6 MB
use std::{cell::RefCell, collections::HashSet, rc::Rc};
struct FindElements {
    root: Option<Rc<RefCell<TreeNode>>>,
    hs: HashSet<i32>,
}

impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut hs = HashSet::new();
        Self::recover(root.as_deref(), 0, &mut hs);
        Self { root, hs }
    }

    fn recover(root: Option<&RefCell<TreeNode>>, x: i32, hs: &mut HashSet<i32>) {
        if let Some(node) = root {
            hs.insert(x);
            let mut node = node.borrow_mut();
            node.val = x;
            Self::recover(node.left.as_deref(), 2 * x + 1, hs);
            Self::recover(node.right.as_deref(), 2 * x + 2, hs);
        }
    }

    fn find(&self, target: i32) -> bool {
        self.hs.contains(&target)
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
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */
// tree hash_table
#[test]
fn test1_1261() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: -1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(-1)))),
    })));
    let fe = FindElements::new(t1);
    assert_eq!(fe.find(1), false);
    assert_eq!(fe.find(2), true);
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: -1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: -1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(-1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(-1)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(-1)))),
    })));
    let fe = FindElements::new(t2);
    assert_eq!(fe.find(1), true);
    assert_eq!(fe.find(3), true);
    assert_eq!(fe.find(5), false);
    let t3 = Some(Rc::new(RefCell::new(TreeNode {
        val: -1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: -1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(-1)))),
                right: None,
            }))),
            right: None,
        }))),
    })));
    let fe = FindElements::new(t3);
    assert_eq!(fe.find(2), true);
    assert_eq!(fe.find(3), false);
    assert_eq!(fe.find(4), false);
    assert_eq!(fe.find(5), true);
}
