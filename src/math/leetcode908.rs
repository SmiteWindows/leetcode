// https://leetcode.com/problems/smallest-range-i/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn smallest_range_i(a: Vec<i32>, k: i32) -> i32 {
    let max = a.iter().max().unwrap();
    let min = a.iter().min().unwrap();
    i32::max(0, max - min - 2 * k)
}
// math
#[test]
fn test1_908() {
    assert_eq!(smallest_range_i(vec![1], 0), 0);
    assert_eq!(smallest_range_i(vec![0, 10], 2), 6);
    assert_eq!(smallest_range_i(vec![1, 3, 6], 3), 0);
}
