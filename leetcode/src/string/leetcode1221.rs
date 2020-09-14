// https://leetcode-cn.com/problems/split-a-string-in-balanced-strings/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn balanced_string_split(s: String) -> i32 {
    let mut l = 0;
    let mut r = 0;
    let mut res = 0;
    for c in s.chars() {
        match c {
            'R' => r += 1,
            'L' => l += 1,
            _ => {}
        }
        if l == r {
            res += 1;
        }
    }
    res
}
// greedy string
#[test]
fn test2_1221() {
    assert_eq!(balanced_string_split("RLRRLLRLRL")), 4);
    assert_eq!(balanced_string_split("RLLLLRRRLR")), 3);
    assert_eq!(balanced_string_split("LLLLRRRR")), 1);
    assert_eq!(balanced_string_split("RLRRRLLRLL")), 2);
}
