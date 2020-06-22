// https://leetcode.com/problems/bulb-switcher/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn bulb_switch(n: i32) -> i32 {
    (n as f64).sqrt() as i32
}
// brainteaser math
#[test]
fn test2_319() {
    assert_eq!(bulb_switch(3), 1);
}
