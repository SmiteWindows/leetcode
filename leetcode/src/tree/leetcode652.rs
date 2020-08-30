// https://leetcode-cn.com/problems/find-duplicate-subtrees/
// Runtime: 12 ms
// Memory Usage: 10.4 MB
use std::{cell::RefCell, collections::HashMap, rc::Rc};
pub fn find_duplicate_subtrees(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let mut count = HashMap::new();
    let mut ans = Vec::new();
    walk(root.as_ref(), &mut count, &mut ans);
    ans
}

fn walk(
    root: Option<&Rc<RefCell<TreeNode>>>,
    count: &mut HashMap<String, i32>,
    ans: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
) -> String {
    if let Some(n) = root {
        let node = n.borrow();
        let serial = node.val.to_string()
            + ","
            + &walk(node.left.as_ref(), count, ans)
            + ","
            + &walk(node.right.as_ref(), count, ans);
        *count.entry(serial.clone()).or_default() += 1;
        if count.get(&serial).unwrap() == &2 {
            ans.push(Some(n.clone()));
        }
        serial
    } else {
        String::from("#")
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
// tree
#[test]
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
    let r2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        right: None,
    })));
    let r1 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let res = vec![r1, r2];
    assert_eq!(res, find_duplicate_subtrees(t));
}
