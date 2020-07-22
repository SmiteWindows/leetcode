// https://leetcode.com/problems/h-index-ii/
// Runtime: 0 ms
// Memory Usage: 2.7 MB
use std::cmp::Ordering::{Equal, Greater, Less};
pub fn h_index(citations: Vec<i32>) -> i32 {
    let n = citations.len();
    let mut l = 0;
    let mut r = n;
    while l < r {
        let m = l + (r - l) / 2;
        match (citations[m] as usize).cmp(&(n - m)) {
            Equal => {
                return (n - m) as i32;
            }
            Less => {
                l = m + 1;
            }
            Greater => {
                r = m;
            }
        }
    }
    (n - l) as i32
}
// binary_search
#[test]
fn test1_275() {
    assert_eq!(h_index(vec![0, 1, 3, 5, 6]), 3);
    assert_eq!(h_index(vec![1]), 1);
}
