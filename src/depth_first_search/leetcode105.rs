// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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
// Runtime: 0 ms
// Memory Usage: 2.8 MB
pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn add_node(
        preorder: &[i32],
        inorder: &[i32],
        map: &HashMap<i32, usize>,
        pre_root_index: usize,
        left: usize,
        right: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if left > right {
            return None;
        }
        let cur = preorder[pre_root_index];
        let mut node = Some(Rc::new(RefCell::new(TreeNode::new(cur))));
        if left == right {
            return node;
        }

        let ino_root_index = map[&cur];
        node.as_mut().unwrap().borrow_mut().left = add_node(
            preorder,
            inorder,
            map,
            pre_root_index + 1,
            left,
            ino_root_index - 1,
        );
        node.as_mut().unwrap().borrow_mut().right = add_node(
            preorder,
            inorder,
            map,
            pre_root_index + 1 + ino_root_index - left,
            ino_root_index + 1,
            right,
        );
        node
    }
    if preorder.is_empty() {
        return None;
    }
    let mut map = HashMap::new();
    for (index, num) in inorder.iter().enumerate() {
        map.insert(*num, index);
    }
    add_node(&preorder, &inorder, &map, 0, 0, preorder.len() - 1)
}
// tree depth_first_search array
#[test]
fn test3_105() {
    let res = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    assert_eq!(
        res,
        build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7])
    );
}
