// https://leetcode.com/problems/find-the-smallest-divisor-given-a-threshold/
// Runtime: 4 ms
// Memory Usage: 2.5 MB
pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
    let mut l = 1;
    let mut r = 1_000_000;
    while l < r {
        let m = l + (r - l) / 2;
        let mut sum = 0;
        for &x in &nums {
            if x % m == 0 {
                sum += x / m;
            } else {
                sum += x / m + 1;
            }
        }
        if sum > threshold {
            l = m + 1;
        } else {
            r = m;
        }
    }
    l
}
// binary_search
#[test]
fn test1_1292() {
    assert_eq!(smallest_divisor(vec![1, 2, 5, 9], 6), 5);
    assert_eq!(smallest_divisor(vec![2, 3, 5, 7, 11], 11), 3);
    assert_eq!(smallest_divisor(vec![19], 5), 4);
}
