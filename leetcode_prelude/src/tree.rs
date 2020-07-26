use std::{cell::RefCell, rc::Rc};

/// Definition for a binary tree node.
///
/// # Note
///
/// I add Ord PartialOrd for sort Vec<TreeNode> when testing
/// Please don't rely on it
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
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

/// Create a binary tree with TreeNode
///
/// # Example
///
/// ```rust
/// use leetcode_prelude::btree;
/// use leetcode_prelude::TreeNode;
/// let tree = btree![1, 2, 3, null, null, 4, 5];
/// println!("{:#?}",tree);
/// ```
#[macro_export]
macro_rules! btree {
    () => {
        None
    };
    ($($e:expr), *) => {
        {
            use std::{cell::RefCell, collections::VecDeque, rc::Rc};

            let elems = vec![$(stringify!($e)), *];
            let elems = elems.iter().map(|n| n.parse::<i32>().ok()).collect::<Vec<_>>();
            let head = Some(Rc::new(RefCell::new(TreeNode::new(elems[0].unwrap()))));
            let mut nodes = VecDeque::new();
            nodes.push_back(head.as_ref().unwrap().clone());

            for i in elems[1..].chunks(2) {
                let node = nodes.pop_front().unwrap();
                if let Some(val) = i[0]{
                    node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    nodes.push_back(node.borrow().left.as_ref().unwrap().clone());
                }
                if i.len() > 1 {
                    if let Some(val) = i[1] {
                        node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                        nodes.push_back(node.borrow().right.as_ref().unwrap().clone());
                    }
                }
            }
            head
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            btree![1, null, 2, null, 3, null, 4, null, null],
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    }))),
                }))),
            })))
        );
        assert_eq!(
            btree![1, null, 2],
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
            btree![1, 2, 3],
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
        assert_eq!(
            btree![-1, 2, 3, null],
            Some(Rc::new(RefCell::new(TreeNode {
                val: -1,
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
}
