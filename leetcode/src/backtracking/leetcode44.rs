// https://leetcode.com/problems/wildcard-matching/
// Runtime: 12 ms
// Memory Usage: 3 MB
pub fn is_match(s: String, p: String) -> bool {
    let n = s.len();
    let m = p.len();
    let s = s.chars().collect::<Vec<_>>();
    let p = p.chars().collect::<Vec<_>>();
    let mut memo = vec![vec![None; m + 1]; n + 1];
    is_match_dp(n, m, &mut memo, &s, &p)
}

fn is_match_dp(
    n: usize,
    m: usize,
    memo: &mut Vec<Vec<Option<bool>>>,
    s: &[char],
    p: &[char],
) -> bool {
    if let Some(ans) = memo[n][m] {
        ans
    } else {
        let res = if n == 0 && m == 0 {
            true
        } else if n == 0 && m != 0 {
            if p[m - 1] == '*' {
                is_match_dp(n, m - 1, memo, s, p)
            } else {
                false
            }
        } else if n != 0 && m == 0 {
            false
        } else if s[n - 1] == p[m - 1] {
            is_match_dp(n - 1, m - 1, memo, s, p)
        } else {
            match p[m - 1] {
                '?' => is_match_dp(n - 1, m - 1, memo, s, p),
                '*' => is_match_dp(n - 1, m, memo, s, p) || is_match_dp(n, m - 1, memo, s, p),
                _ => false,
            }
        };
        memo[n][m] = Some(res);
        res
    }
}
// string dynamic_programming backtracking greedy
#[test]
fn test1_44() {
    assert_eq!(is_match(String::from("aa"), String::from("a")), false);
    assert_eq!(is_match(String::from("aa"), String::from("*")), true);
    assert_eq!(is_match(String::from("cb"), String::from("?a")), false);
    assert_eq!(is_match(String::from("adceb"), String::from("a*b")), true);
    assert_eq!(
        is_match(String::from("acdcb"), String::from("a*c?b")),
        false
    );
}
