// https://leetcode-cn.com/problems/flatten-nested-list-iterator/
// Runtime: 4 ms
// Memory Usage: 2.9 MB
use std::{iter::Peekable, vec::IntoIter};
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
struct NestedIterator {
    it: Peekable<IntoIter<i32>>,
}

trait ToVec {
    fn into_vec(self) -> Vec<i32>;
}

impl ToVec for NestedInteger {
    fn into_vec(self) -> Vec<i32> {
        match self {
            NestedInteger::Int(x) => vec![x],
            NestedInteger::List(v) => {
                let mut res = Vec::new();
                for x in v {
                    res.append(&mut x.into_vec());
                }
                res
            }
        }
    }
}

impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut data = Vec::new();
        for x in nested_list {
            data.append(&mut x.into_vec());
        }
        Self {
            it: data.into_iter().peekable(),
        }
    }

    fn next(&mut self) -> i32 {
        self.it.next().unwrap()
    }

    fn has_next(&mut self) -> bool {
        self.it.peek().is_some()
    }
}
/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
// design stack
#[test]
fn test1_341() {
    let nested_list = vec![
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        NestedInteger::Int(2),
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
    ];
    let mut obj = NestedIterator::new(nested_list);
    let res = vec![1, 1, 2, 1, 1];
    let mut ans = Vec::new();
    while obj.has_next() {
        ans.push(obj.next());
    }
    assert_eq!(ans, res);

    let nested_list = vec![
        NestedInteger::Int(1),
        NestedInteger::List(vec![
            NestedInteger::Int(4),
            NestedInteger::List(vec![NestedInteger::Int(6)]),
        ]),
    ];
    let mut obj = NestedIterator::new(nested_list);
    let res = vec![1, 4, 6];
    let mut ans = Vec::new();
    while obj.has_next() {
        ans.push(obj.next());
    }
    assert_eq!(ans, res);
}
