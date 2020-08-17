// https://leetcode.com/problems/minimum-operations-to-make-array-equal/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn min_operations(n: i32) -> i32 {
    let mut res = 0;
    let mut i = 1;
    while i < n {
        res += n - i;
        i += 2;
    }
    res
}
// math
#[test]
fn test1_1551() {
    assert_eq!(min_operations(3), 2);
    assert_eq!(min_operations(6), 9);
}
