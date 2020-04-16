// https://leetcode.com/problems/unique-binary-search-trees/
use std::convert::TryInto;
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn num_trees(n: i32) -> i32 {
    let mut catalan = 1 as i64;
    let n = n as i64;
    for i in 0..n {
        catalan = catalan * 2 * (2 * i + 1) / (i + 2);
    }
    catalan.try_into().unwrap()
}
// tree dynamic_programming
#[test]
fn test1_96() {
    assert_eq!(num_trees(3), 5);
    assert_eq!(num_trees(19), 1767263190);
}
