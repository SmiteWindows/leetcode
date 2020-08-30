// https://leetcode-cn.com/problems/binary-gap/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn binary_gap(n: i32) -> i32 {
    let mut max = 0;
    let mut prev = None;
    for i in 0..32 {
        let bit = 1 << i;
        if n & bit != 0 {
            if let Some(j) = prev {
                max = max.max(i - j);
            }
            prev = Some(i);
        }
    }
    max
}
// math
#[test]
fn test1_868() {
    assert_eq!(binary_gap(22), 2);
    assert_eq!(binary_gap(5), 2);
    assert_eq!(binary_gap(6), 1);
    assert_eq!(binary_gap(8), 0);
}
