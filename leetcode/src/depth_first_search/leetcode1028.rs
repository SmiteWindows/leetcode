// https://leetcode-cn.com/problems/recover-a-tree-from-preorder-traversal/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cell::RefCell, iter::Peekable, rc::Rc, str::Chars, vec::IntoIter};
pub fn recover_from_preorder(s: String) -> Option<Rc<RefCell<TreeNode>>> {
    let toks = parse_tokens(&mut s.chars().peekable());
    parse_root(&mut toks.into_iter().peekable())
}

fn parse_tokens(it: &mut Peekable<Chars<'_>>) -> Vec<Tok> {
    let mut toks = Vec::new();
    while let Some(c) = it.next() {
        match c {
            '-' => {
                let mut d = 1;
                while let Some('-') = it.peek() {
                    it.next();
                    d += 1;
                }
                toks.push(Tok::D(d));
            }
            '0'..='9' => {
                let mut n = (c as u8 - b'0') as i32;
                while let Some('0'..='9') = it.peek() {
                    n *= 10;
                    n += (it.next().unwrap() as u8 - b'0') as i32;
                }
                toks.push(Tok::N(n));
            }
            _ => {}
        }
    }
    toks
}

fn parse(it: &mut Peekable<IntoIter<Tok>>, depth: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(&Tok::D(d)) = it.peek() {
        if d == depth {
            it.next();
            if let Some(Tok::N(n)) = it.next() {
                Some(Rc::new(RefCell::new(TreeNode {
                    val: n,
                    left: parse(it, d + 1),
                    right: parse(it, d + 1),
                })))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn parse_root(it: &mut Peekable<IntoIter<Tok>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(Tok::N(n)) = it.next() {
        Some(Rc::new(RefCell::new(TreeNode {
            val: n,
            left: parse(it, 1),
            right: parse(it, 1),
        })))
    } else {
        None
    }
}

enum Tok {
    N(i32),
    D(usize),
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
fn test2_1028() {
    let res1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    let res2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: None,
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: None,
            }))),
            right: None,
        }))),
    })));
    let res3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 401,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 349,
                left: Some(Rc::new(RefCell::new(TreeNode::new(90)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(88)))),
        }))),
        right: None,
    })));
    assert_eq!(recover_from_preorder("1-2--3--4-5--6--7".to_string()), res1);
    assert_eq!(
        recover_from_preorder("1-2--3---4-5--6---7".to_string()),
        res2
    );
    assert_eq!(
        recover_from_preorder("1-401--349---90--88".to_string()),
        res3
    );
}
