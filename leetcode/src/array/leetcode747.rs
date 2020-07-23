// https://leetcode.com/problems/largest-number-at-least-twice-of-others/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn dominant_index(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 1 {
        return 0;
    }
    let mut copy = nums.clone();
    copy.sort();
    if copy[n - 1] < 2 * copy[n - 2] {
        -1
    } else {
        nums.iter().position(|&x| x == copy[n - 1]).unwrap() as i32
    }
}
// array
#[test]
fn test1_747() {
    assert_eq!(dominant_index(vec![3, 6, 1, 0]), 1);
    assert_eq!(dominant_index(vec![1, 2, 3, 4]), -1);
}
