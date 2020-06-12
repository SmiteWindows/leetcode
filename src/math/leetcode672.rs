// https://leetcode.com/problems/bulb-switcher-ii/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn flip_lights(n: i32, m: i32) -> i32 {
    let n = n.min(3);
    if m == 0 || n == 0 {
        return 1;
    }
    if n == 1 {
        return 2;
    }
    if n == 2 {
        return if m == 1 { 3 } else { 4 };
    }
    if m == 1 {
        return 4;
    }
    if m == 2 {
        7
    } else {
        8
    }
}
// math
#[test]
fn test1_672() {
    assert_eq!(flip_lights(1, 1), 2);
    assert_eq!(flip_lights(2, 1), 3);
    assert_eq!(flip_lights(3, 1), 4);
}
