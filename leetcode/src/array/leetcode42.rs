// https://leetcode-cn.com/problems/trapping-rain-water/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    if n == 0 {
        return 0;
    }
    let mut l = 0;
    let mut r = n - 1;
    let mut level = 0;
    let mut res = 0;
    while l < r {
        if height[l] < height[r] {
            let lower = height[l];
            level = level.max(lower);
            res += level - lower;
            l += 1;
        } else {
            let lower = height[r];
            level = level.max(lower);
            res += level - lower;
            r -= 1;
        }
    }
    res
}
// array two_pointers stack
#[test]
fn test2_42() {
    assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
}
