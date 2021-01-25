// https://leetcode-cn.com/problems/reach-a-number/
// Runtime: 0 ms
// Memory Usage: 2.4 MB
pub fn reach_number(target: i32) -> i32 {
    let target = target.abs();
    let n = (((2 * target as i64) as f64 + 0.25).sqrt() - 0.5).ceil() as i32;
    let sum = n * (n + 1) / 2;
    if sum == target {
        n
    } else {
        let diff = sum - target;
        if diff % 2 == 0 {
            n
        } else if n % 2 == 0 {
            n + 1
        } else {
            n + 2
        }
    }
}
// math
#[test]
fn test1_754() {
    assert_eq!(reach_number(3), 2);
    assert_eq!(reach_number(2), 3);
}
