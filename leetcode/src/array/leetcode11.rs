// https://leetcode.com/problems/container-with-most-water/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
// ✔
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = height.len() - 1;
    let mut max = 0;

    while l < r {
        max = max.max(height[l].min(height[r]) * (r - l) as i32);

        if height[l] < height[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }
    max
}
// array two_pointers
#[test]
fn test2_11() {
    assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
}
