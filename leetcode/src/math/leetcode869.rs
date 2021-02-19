// https://leetcode-cn.com/problems/reordered-power-of-2/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashSet;
pub fn reordered_power_of2(n: i32) -> bool {
    let mut hs = HashSet::new();
    for i in 0..32 {
        let mut v = (1 << i).to_string().chars().collect::<Vec<_>>();
        v.sort_unstable();
        hs.insert(v);
    }
    let mut v = n.to_string().chars().collect::<Vec<_>>();
    v.sort_unstable();
    hs.contains(&v)
}
// math
#[test]
fn test1_869() {
    assert_eq!(reordered_power_of2(1), true);
    assert_eq!(reordered_power_of2(10), false);
    assert_eq!(reordered_power_of2(16), true);
    assert_eq!(reordered_power_of2(24), false);
    assert_eq!(reordered_power_of2(46), true);
}
