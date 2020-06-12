// https://leetcode.com/problems/wiggle-subsequence/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    use std::cmp::Ordering::{Equal, Greater, Less};
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    let mut up = vec![1];
    let mut down = vec![1];
    for i in 1..n {
        match nums[i].cmp(&nums[i - 1]) {
            Greater => {
                down.push(up[i - 1] + 1);
                up.push(up[i - 1]);
            }
            Less => {
                down.push(down[i - 1]);
                up.push(down[i - 1] + 1);
            }
            Equal => {
                down.push(down[i - 1]);
                up.push(up[i - 1]);
            }
        }
    }
    up.into_iter().chain(down.into_iter()).max().unwrap()
}
// greedy dynamic_programming
#[test]
fn test2_376() {
    assert_eq!(wiggle_max_length(vec![1, 7, 4, 9, 2, 5]), 6);
    assert_eq!(
        wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8]),
        7
    );
    assert_eq!(wiggle_max_length(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 2);
}
