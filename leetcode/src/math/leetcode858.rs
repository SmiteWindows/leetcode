// https://leetcode-cn.com/problems/mirror-reflection/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
pub fn mirror_reflection(mut p: i32, mut q: i32) -> i32 {
    while p % 2 == 0 && q % 2 == 0 {
        p /= 2;
        q /= 2;
    }
    if p % 2 == 0 {
        return 2;
    }
    if q % 2 == 0 {
        return 0;
    }
    1
}
// math
#[test]
fn test1_858() {
    assert_eq!(mirror_reflection(2, 1), 2);
}
