// https://leetcode.com/problems/minimum-size-subarray-sum/
// Runtime: 0 ms
// Memory Usage: 2.4 MB
pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut res = usize::MAX;
    let mut sum = 0;
    let mut l = 0;
    for r in 0..n {
        sum += nums[r];
        while sum >= s {
            res = res.min(r - l + 1);
            sum -= nums[l];
            l += 1;
        }
    }
    if res == usize::MAX {
        0
    } else {
        res as i32
    }
}
// array two_pointers binary_search
#[test]
fn test1_209() {
    assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
}
