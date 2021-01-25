// https://leetcode-cn.com/problems/monotone-increasing-digits/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn monotone_increasing_digits(n: i32) -> i32 {
    let mut s = n.to_string().chars().collect::<Vec<_>>();
    let n = s.len();
    let mut i = 1;
    while i < n && s[i] >= s[i - 1] {
        i += 1;
    }
    while i > 0 && i < n && s[i - 1] > s[i] {
        s[i - 1] = (s[i - 1] as u8 - 1) as char;
        i -= 1;
    }
    while i + 1 < n {
        s[i + 1] = '9';
        i += 1;
    }
    s.into_iter().collect::<String>().parse::<i32>().unwrap()
}
// greedy
#[test]
fn test1_738() {
    assert_eq!(monotone_increasing_digits(10), 9);
    assert_eq!(monotone_increasing_digits(1234), 1234);
    assert_eq!(monotone_increasing_digits(332), 299);
}
