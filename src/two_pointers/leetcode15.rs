// https://leetcode.com/problems/3sum/
// Runtime: 16 ms
// Memory Usage: 3.4 MB
use std::cmp::Ordering::{Equal, Greater, Less};
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let n = nums.len();
    if n < 3 {
        return res;
    }
    let mut nums = nums;
    nums.sort_unstable();
    for i in 0..n - 2 {
        let ai = nums[i];
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        if nums[i] > 0 {
            break;
        }
        let mut j = i + 1;
        let mut k = n - 1;
        while j < k {
            let bj = nums[j];
            let ck = nums[k];
            let sum = ai + bj + ck;
            match sum.cmp(&0) {
                Equal => {
                    res.push(vec![ai, bj, ck]);
                    j += 1;
                    k -= 1;
                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                    while j < k && nums[k] == nums[k + 1] {
                        k -= 1;
                    }
                }
                Less => j += 1,
                Greater => k -= 1,
            }
        }
    }
    res
}
// array two_pointers
#[test]
fn test1_15() {
    assert_eq!(
        three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![vec![-1, 0, 1], vec![-1, -1, 2]]
    );
}
