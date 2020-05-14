// https://leetcode.com/problems/k-diff-pairs-in-an-array/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::{HashMap, HashSet},
};
pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
    let mut res = 0;
    let mut hs = HashSet::new();
    let mut hm = HashMap::new();
    match k.cmp(&0) {
        Equal => {
            for x in nums {
                let e = hm.entry(x).or_default();
                if let 1 = *e {
                    res += 1;
                }
                *e += 1;
            }
        }
        Greater => {
            for x in nums {
                if hs.contains(&x) {
                    continue;
                } else {
                    hs.insert(x);
                    if hs.contains(&(x + k)) {
                        res += 1;
                    }
                    if hs.contains(&(x - k)) {
                        res += 1;
                    }
                }
            }
        }
        Less => {}
    }
    res
}
// array two_pointers
#[test]
fn test2_532() {
    assert_eq!(find_pairs(vec![3, 1, 4, 1, 5], 2), 2);
    assert_eq!(find_pairs(vec![1, 2, 3, 4, 5], 1), 4);
    assert_eq!(find_pairs(vec![1, 3, 1, 5, 4], 0), 1);
}
