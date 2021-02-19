// https://leetcode-cn.com/problems/two-sum-ii-input-array-is-sorted/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::cmp::Ordering::{Equal, Greater, Less};
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut l = 0_usize;
    let mut r = numbers.len() - 1;
    while l < r {
        match (numbers[l] + numbers[r]).cmp(&target) {
            Equal => return vec![(l + 1) as i32, (r + 1) as i32],
            Less => {
                l += 1;
            }
            Greater => {
                r -= 1;
            }
        }
    }
    vec![0, 0]
}
// array two_pointers binary_search
#[test]
fn test1_167() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
}
