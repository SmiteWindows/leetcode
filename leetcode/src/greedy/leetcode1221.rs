// https://leetcode-cn.com/problems/split-a-string-in-balanced-strings/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn balanced_string_split(s: String) -> i32 {
    let mut balance = 0;
    let mut res = 0;
    for c in s.chars() {
        match c {
            'R' => balance += 1,
            'L' => balance -= 1,
            _ => {}
        }
        if balance == 0 {
            res += 1;
        }
    }
    res
}
// greedy string
#[test]
fn test1_1221() {
    assert_eq!(balanced_string_split("RLRRLLRLRL".to_string()), 4);
    assert_eq!(balanced_string_split("RLLLLRRRLR".to_string()), 3);
    assert_eq!(balanced_string_split("LLLLRRRR".to_string()), 1);
    assert_eq!(balanced_string_split("RLRRRLLRLL".to_string()), 2);
}
