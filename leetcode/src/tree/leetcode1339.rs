// https://leetcode-cn.com/problems/maximum-product-of-splitted-binary-tree/
// Runtime: 20 ms
// Memory Usage: 10.2 MB
use std::{cell::RefCell, rc::Rc};
pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let sum = postorder_sum(root.as_deref());
    let mut res = 0;
    postorder_product(root.as_deref(), &mut res, sum);
    (res % 1_000_000_007) as i32
}

fn postorder_sum(root: Option<&RefCell<TreeNode>>) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        let left = postorder_sum(node.left.as_deref());
        let right = postorder_sum(node.right.as_deref());
        val + left + right
    } else {
        0
    }
}

fn postorder_product(root: Option<&RefCell<TreeNode>>, max: &mut i64, sum: i32) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let val = node.val;
        let left = postorder_product(node.left.as_deref(), max, sum);
        let right = postorder_product(node.right.as_deref(), max, sum);
        let res = val + left + right;
        *max = (*max).max((sum - res) as i64 * res as i64);
        res
    } else {
        0
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
// tree depth_first_search dynamic_programming
#[test]
fn test1_1339() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: None,
        }))),
    }))); // [1,2,3,4,5,6]
    assert_eq!(max_product(t1), 110);
    let t2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            }))),
        }))),
    }))); // [1,null,2,3,4,null,null,5,6]
    assert_eq!(max_product(t2), 90);
    let t3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode::new(11)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
        }))),
    }))); // [2,3,9,10,7,8,6,5,4,11,1]
    assert_eq!(max_product(t3), 1025);
    let t4 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: None,
    })));
    assert_eq!(max_product(t4), 1);
}
