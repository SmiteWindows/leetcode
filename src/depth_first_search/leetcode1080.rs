// https://leetcode.com/problems/insufficient-nodes-in-root-to-leaf-paths/
// Runtime: 8 ms
// Memory Usage: 2.5 MB
use std::{cell::RefCell, rc::Rc};
pub fn sufficient_subset(
    root: Option<Rc<RefCell<TreeNode>>>,
    limit: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    postorder(root, limit)
}

fn postorder(root: Option<Rc<RefCell<TreeNode>>>, limit: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        let val = node.borrow_mut().val;
        let left = node.borrow_mut().left.take();
        let right = node.borrow_mut().right.take();
        if left.is_none() && right.is_none() {
            if val >= limit {
                Some(node)
            } else {
                None
            }
        } else {
            let l = postorder(left, limit - val);
            let r = postorder(right, limit - val);
            if l.is_some() || r.is_some() {
                node.borrow_mut().left = l;
                node.borrow_mut().right = r;
                Some(node)
            } else {
                None
            }
        }
    } else {
        None
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
// depth_first_search
#[test]
fn test1_1080() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -99,
                left: Some(Rc::new(RefCell::new(TreeNode::new(-99)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(-99)))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -99,
                left: Some(Rc::new(RefCell::new(TreeNode::new(12)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode::new(-99)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(14)))),
            }))),
        }))),
    }))); // [1,2,3,4,-99,-99,7,8,9,-99,-99,12,13,-99,14]
    let res1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(14)))),
            }))),
        }))),
    }))); // [1,2,3,4,null,null,7,8,9,null,14]
    assert_eq!(sufficient_subset(t1, 1), res1);
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 11,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode::new(17)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
        }))),
    }))); // [5,4,8,11,null,17,4,7,1,null,null,5,3]
    let res2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 11,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: None,
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode::new(17)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                right: None,
            }))),
        }))),
    }))); // [5,4,8,11,null,17,4,7,null,null,null,5]
    assert_eq!(sufficient_subset(t2, 22), res2);
    let t3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(-5)))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: -3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: None,
        }))),
    }))); // [1,2,-3,-5,null,4,null]
    let res3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: -3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: None,
        }))),
    }))); // [1,null,-3,4]
    assert_eq!(sufficient_subset(t3, -1), res3);
}
