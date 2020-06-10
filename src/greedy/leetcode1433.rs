// https://leetcode.com/problems/check-if-a-string-can-break-another-string/
// Runtime: 12 ms
// Memory Usage: 3.1 MB
pub fn check_if_can_break(s1: String, s2: String) -> bool {
    let n = s1.len();
    let mut s1 = s1.chars().collect::<Vec<_>>();
    let mut s2 = s2.chars().collect::<Vec<_>>();
    s1.sort_unstable();
    s2.sort_unstable();
    let mut sum1 = 0;
    let mut sum2 = 0;
    for i in 0..n {
        if s1[i] <= s2[i] {
            sum1 += 1;
        }
        if s1[i] >= s2[i] {
            sum2 += 1;
        }
    }
    sum1 == n || sum2 == n
}
// string greedy
#[test]
fn test2_1433() {
    assert_eq!(
        check_if_can_break(String::from("abc"), String::from("xya")),
        true
    );
    assert_eq!(
        check_if_can_break(String::from("abe"), String::from("acd")),
        false
    );
    assert_eq!(
        check_if_can_break(String::from("leetcodee"), String::from("interview")),
        true
    );
}
