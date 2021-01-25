// https://leetcode-cn.com/problems/bitwise-ors-of-subarrays/
// Runtime: 288 ms
// Memory Usage: 7.3 MB
use std::collections::HashSet;
pub fn subarray_bitwise_o_rs(a: Vec<i32>) -> i32 {
    let mut res = HashSet::new();
    let mut prev = HashSet::new();
    for x in a {
        let mut cur = HashSet::new();
        cur.insert(x);
        for y in prev {
            cur.insert(y | x);
        }
        for &x in &cur {
            res.insert(x);
        }
        prev = cur;
    }
    res.len() as i32
}
// bit_manipulation dynamic_programming
#[test]
fn test2_898() {
    assert_eq!(subarray_bitwise_o_rs(vec![0]), 1);
    assert_eq!(subarray_bitwise_o_rs(vec![1, 1, 2]), 3);
    assert_eq!(subarray_bitwise_o_rs(vec![1, 2, 4]), 6);
}
