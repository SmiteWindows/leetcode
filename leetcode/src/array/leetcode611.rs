// https://leetcode-cn.com/problems/valid-triangle-number/
// Runtime: 8 ms
// Memory Usage: 2 MB
pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let n = nums.len();
    nums.sort_unstable();
    for i in (2..n).rev() {
        let mut l = 0;
        let mut r = i - 1;
        while l < r {
            if nums[l] + nums[r] > nums[i] {
                res += r - l;
                r -= 1;
            } else {
                l += 1;
            }
        }
    }
    res as i32
}
// array
#[test]
fn test1_611() {
    assert_eq!(triangle_number(vec![2, 2, 3, 4]), 3);
}
