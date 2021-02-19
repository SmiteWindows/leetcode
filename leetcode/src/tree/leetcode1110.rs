// https://leetcode-cn.com/problems/delete-nodes-and-return-forest/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, collections::HashSet, iter::FromIterator, rc::Rc};
pub fn del_nodes(
    root: Option<Rc<RefCell<TreeNode>>>,
    to_delete: Vec<i32>,
) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let mut res = Vec::new();
    let nodes = HashSet::from_iter(to_delete);
    let (root, forest) = postorder(root, &nodes);
    if root.is_some() {
        res.push(root);
    }
    for t in forest {
        res.push(t);
    }
    res
}

type RootForest = (
    Option<Rc<RefCell<TreeNode>>>,
    Vec<Option<Rc<RefCell<TreeNode>>>>,
);

fn postorder(root: Option<Rc<RefCell<TreeNode>>>, nodes: &HashSet<i32>) -> RootForest {
    if let Some(node) = root {
        let val = node.borrow_mut().val;
        let left = node.borrow_mut().left.take();
        let right = node.borrow_mut().right.take();
        let (left_root, left_forest) = postorder(left, nodes);
        let (right_root, right_forest) = postorder(right, nodes);
        let mut forest = Vec::new();
        for t in left_forest {
            forest.push(t);
        }
        for t in right_forest {
            forest.push(t);
        }
        if nodes.contains(&val) {
            if left_root.is_some() {
                forest.push(left_root);
            }
            if right_root.is_some() {
                forest.push(right_root);
            }
            (None, forest)
        } else {
            node.borrow_mut().left = left_root;
            node.borrow_mut().right = right_root;
            (Some(node), forest)
        }
    } else {
        (None, vec![])
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
// tree depth_first_search
#[test]
fn test1_1110() {
    use leetcode_prelude::{btree, vec_btree};
    assert_eq!(
        del_nodes(btree![1, 2, 3, 4, 5, 6, 7], vec![3, 5]),
        vec_btree![[1, 2, null, 4], [6], [7]]
    );
}
