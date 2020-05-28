// https://leetcode.com/problems/path-sum-iii/
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
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::{cell::RefCell, collections::HashMap, rc::Rc};
pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
    let mut map = HashMap::new();
    map.insert(0, 1);
    walk(root.as_deref(), sum, 0, &mut map)
}

fn walk(
    root: Option<&RefCell<TreeNode>>,
    sum: i32,
    mut curr_sum: i32,
    map: &mut HashMap<i32, i32>,
) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let mut res = 0;
        curr_sum += node.val;
        res += *map.entry(curr_sum - sum).or_default();
        *map.entry(curr_sum).or_default() += 1;
        res += walk(node.right.as_deref(), sum, curr_sum, map);
        res += walk(node.left.as_deref(), sum, curr_sum, map);
        map.entry(curr_sum).and_modify(|e| *e -= 1);
        res
    } else {
        0
    }
}
// tree
#[test]
fn test1_437() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 10,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(-2)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: -3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(11)))),
        }))),
    })));
    assert_eq!(3, path_sum(root, 8));
}
