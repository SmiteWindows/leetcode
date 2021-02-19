// https://leetcode-cn.com/problems/count-odd-numbers-in-an-interval-range/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn count_odds(low: i32, high: i32) -> i32 {
    let l = low / 2 * 2;
    let r = (high + 1) / 2 * 2;
    (r - l) / 2
}
// math
#[test]
fn test1_1523() {
    assert_eq!(count_odds(3, 7), 3);
    assert_eq!(count_odds(8, 10), 1);
}
