// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<Self>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}

pub fn as_list(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut l = None;
    for i in v.into_iter().rev() {
        l = Some(Box::new(ListNode { next: l, val: i }));
    }
    l
}
#[test]
fn test() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let l = as_list(v.clone());
    let mut cur = &l;
    for i in v {
        assert_eq!(i, cur.as_deref().expect("exist").val);
        cur = &cur.as_deref().expect("exist").next;
    }
}

use std::{cell::RefCell, collections::VecDeque, rc::Rc};
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

pub fn as_tree(v: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    let head = Some(Rc::new(RefCell::new(TreeNode::new(v[0].expect("exist")))));
    let mut deq = VecDeque::new();
    deq.push_back(head.as_ref().expect("exist").clone());
    for children in v[1..].chunks(2) {
        let parent = deq.pop_front().expect("exist");
        if let Some(v) = children[0] {
            let tmp = Some(Rc::new(RefCell::new(TreeNode::new(v))));
            parent.borrow_mut().left = tmp;
            deq.push_back(parent.borrow().left.as_ref().expect("exist").clone());
        }
        if children.len() > 1 && children[1].is_some() {
            let tmp = Some(Rc::new(RefCell::new(TreeNode::new(
                children[1].expect("exist"),
            ))));
            parent.borrow_mut().right = tmp;
            deq.push_back(parent.borrow().right.as_ref().expect("exist").clone());
        }
    }
    head
}

#[test]
fn tree_test() {
    assert_eq!(
        as_tree(vec![Some(1), None, Some(2)]),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })))
        })))
    );
    assert_eq!(
        as_tree(vec![Some(1), Some(2), Some(3)]),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            })))
        })))
    );
}
