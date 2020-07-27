// https://leetcode.com/problems/number-of-good-leaf-nodes-pairs/
// Runtime: 4 ms
// Memory Usage: 2.6 MB
use std::{cell::RefCell, rc::Rc};
pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
    let mut res = 0;
    if distance < 2 {
        return 0;
    }
    postorder(root, &mut res, distance);
    res as i32
}

fn postorder(root: Option<Rc<RefCell<TreeNode>>>, all: &mut usize, distance: i32) -> [usize; 10] {
    if let Some(node) = root {
        let mut res = [0; 10];
        let mut node = node.borrow_mut();
        let mut left = node.left.take();
        let mut right = node.right.take();
        if let (None, None) = (left.as_mut(), right.as_mut()) {
            res[0] = 1;
        } else {
            let l = postorder(left, all, distance);
            let r = postorder(right, all, distance);
            for (i, &li) in l.iter().enumerate().take(9) {
                for (j, &rj) in r.iter().enumerate().take(9) {
                    if i + j <= (distance - 2) as usize {
                        *all += li * rj;
                    }
                }
            }
            for i in 0..9 {
                res[i + 1] += l[i];
            }
            for i in 0..9 {
                res[i + 1] += r[i];
            }
        }
        res
    } else {
        [0; 10]
    }
}

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
fn test1_1530() {
    use leetcode_prelude::btree;
    assert_eq!(count_pairs(btree![1, 2, 3, null, 4], 3), 1);
    assert_eq!(count_pairs(btree![1, 2, 3, 4, 5, 6, 7], 3), 2);
    assert_eq!(
        count_pairs(
            btree![7, 1, 4, 6, null, 5, 3, null, null, null, null, null, 2],
            3
        ),
        1
    );
    assert_eq!(count_pairs(btree![100], 1), 0);
    assert_eq!(count_pairs(btree![1, 1, 1], 2), 1);
}
