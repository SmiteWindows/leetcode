// https://leetcode-cn.com/problems/climbing-stairs/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn climb_stairs(n: i32) -> i32 {
    match n {
        1 | 2 => n,
        k => (2..k).fold((1, 2), |(a, b), _| (b, a + b)).1,
    }
}
// dynamic_programming
#[test]
fn test1_70() {
    assert_eq!(climb_stairs(2), 2);
    assert_eq!(climb_stairs(3), 3);
}
