// https://leetcode-cn.com/problems/power-of-three/
// Runtime: 0 ms
// Memory Usage: 2.8 MB
pub fn is_power_of_three(n: i32) -> bool {
    let n = n as f64;
    n.log10() / 3.0_f64.log10() % 1.0 == 0.0
}
// math
#[test]
fn test1_326() {
    assert_eq!(is_power_of_three(27), true);
    assert_eq!(is_power_of_three(0), false);
    assert_eq!(is_power_of_three(9), true);
    assert_eq!(is_power_of_three(45), false);
}
