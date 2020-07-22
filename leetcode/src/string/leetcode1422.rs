// https://leetcode.com/problems/maximum-score-after-splitting-a-string/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn max_score(s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let n = s.len();
    let mut zeros = vec![0; n + 1];
    for i in 0..n {
        zeros[i + 1] = zeros[i] + if s[i] == '0' { 1 } else { 0 };
    }

    let mut ones = vec![0; n + 1];
    for i in (0..n).rev() {
        ones[i] = ones[i + 1] + if s[i] == '1' { 1 } else { 0 };
    }
    let mut res = 0;
    for i in 1..n {
        res = res.max(zeros[i] + ones[i]);
    }
    res
}
// string
#[test]
fn test1_1422() {
    assert_eq!(max_score(String::from("011101")), 5);
    assert_eq!(max_score(String::from("00111")), 5);
    assert_eq!(max_score(String::from("1111")), 3);
}
