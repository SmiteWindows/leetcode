// https://leetcode.com/problems/find-duplicate-subtrees/
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
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub fn find_duplicate_subtrees(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    fn walk(
        root: Option<&Rc<RefCell<TreeNode>>>,
        trees: &mut HashMap<String, i32>,
        count: &mut HashMap<i32, i32>,
        ans: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        t: i32,
    ) -> i32 {
        if let Some(n) = root {
            let node = n.borrow();
            let serial = node.val.to_string()
                + ","
                + &walk(node.left.as_ref(), trees, count, ans, t).to_string()
                + ","
                + &walk(node.right.as_ref(), trees, count, ans, t).to_string();
            let uid = *trees.entry(serial).or_insert(t + 1);
            let v = *count.entry(uid).or_default() + 1;
            count.insert(uid, v);
            if count.get(&uid).unwrap() == &2 {
                dbg!();
                ans.push(Some(n.clone()));
            }
            uid
        } else {
            0
        }
    }

    let mut trees = HashMap::new();
    let mut count = HashMap::new();
    let mut ans = Vec::new();
    walk(root.as_ref(), &mut trees, &mut count, &mut ans, 1);
    ans
}
// tree
#[test]
#[ignore]
fn test1_652() {
    let t = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
    })));
    let r1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        right: None,
    })));
    let r2 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let res = vec![r1, r2];
    assert_eq!(res, find_duplicate_subtrees(t));
}
