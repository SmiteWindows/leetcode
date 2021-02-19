// https://leetcode-cn.com/problems/minimum-insertions-to-balance-a-parentheses-string/
// Runtime: 4 ms
// Memory Usage: 2.4 MB
pub fn min_insertions(s: String) -> i32 {
    let mut right = 0;
    let mut res = 0;
    for c in s.chars().rev() {
        if c == ')' {
            right += 1;
        } else if right >= 2 {
            right -= 2;
            if right % 2 == 1 {
                right += 1;
                res += 1;
            }
        } else if right == 0 {
            res += 2;
        } else {
            res += 1;
            right -= 1;
        }
    }
    res += right / 2 + (right % 2) * 2;
    res
}
// string stack
#[test]
fn test2_1541() {
    assert_eq!(min_insertions("(()))".to_string()), 1);
    assert_eq!(min_insertions("())".to_string()), 0);
    assert_eq!(min_insertions("))())(".to_string()), 3);
    assert_eq!(min_insertions("((((((".to_string()), 12);
    assert_eq!(min_insertions(")))))))".to_string()), 5);
}
