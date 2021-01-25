// https://leetcode-cn.com/problems/clumsy-factorial/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn clumsy(n: i32) -> i32 {
    let magic = vec![1, 2, 2, -1, 0, 0, 3, 3];
    n + if n > 4 {
        magic[(n % 4) as usize]
    } else {
        magic[(n + 3) as usize]
    }
}
// math
#[test]
fn test1_1006() {
    assert_eq!(clumsy(4), 7);
    assert_eq!(clumsy(10), 12);
}
