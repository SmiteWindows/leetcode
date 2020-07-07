// https://leetcode.com/problems/patching-array/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
    let mut res = 0;
    let mut miss = 1_i64;
    let mut i = 0;
    while miss <= n as i64 {
        if i < nums.len() && nums[i] as i64 <= miss {
            miss += nums[i] as i64;
            i += 1;
        } else {
            miss += miss;
            res += 1;
        }
    }
    res
}
// greedy
#[test]
fn test1_330() {
    assert_eq!(min_patches(vec![1, 3], 6), 1);
    assert_eq!(min_patches(vec![1, 5, 10], 20), 2);
    assert_eq!(min_patches(vec![1, 2, 2], 5), 0);
}
