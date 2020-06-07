// https://leetcode.com/problems/regular-expression-matching/
// Runtime: 0 ms
// Memory Usage: 2 MB
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
        } else if n != 0 && m == 0 {
            false
        } else if n == 0 && m != 0 {
            if p[m - 1] == '*' {
                is_match_dp(n, m - 2, memo, s, p)
            } else {
                false
            }
        } else {
            if s[n - 1] == p[m - 1] {
                is_match_dp(n - 1, m - 1, memo, s, p)
            } else {
                match p[m - 1] {
                    '*' => match p[m - 2] {
                        '*' => false,
                        '.' => {
                            is_match_dp(n - 1, m, memo, s, p) || is_match_dp(n, m - 2, memo, s, p)
                        }
                        _ => {
                            if s[n - 1] != p[m - 2] {
                                is_match_dp(n, m - 2, memo, s, p)
                            } else {
                                is_match_dp(n - 1, m, memo, s, p)
                                    || is_match_dp(n, m - 2, memo, s, p)
                            }
                        }
                    },
                    '.' => is_match_dp(n - 1, m - 1, memo, s, p),
                    _ => false,
                }
            }
        };
        memo[n][m] = Some(res);
        res
    }
}
// string dynamic_programming backtracking
#[test]
fn test1_10() {
    assert_eq!(is_match(String::from("aa"), String::from("a")), false);
    assert_eq!(is_match(String::from("aa"), String::from("a*")), true);
    assert_eq!(is_match(String::from("ab"), String::from(".*")), true);
    assert_eq!(is_match(String::from("aab"), String::from("c*a*b")), true);
    assert_eq!(
        is_match(String::from("mississippi"), String::from("mis*is*p*.")),
        false
    );
}
