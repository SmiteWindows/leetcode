// https://leetcode-cn.com/problems/number-of-substrings-with-only-1s/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
const MOD: i64 = 1_000_000_007;
pub fn num_sub(s: String) -> i32 {
    let mut res: i64 = 0;
    let mut it = s.chars().peekable();
    while let Some(c) = it.next() {
        if c == '1' {
            let mut n = 1;
            while let Some('1') = it.peek() {
                it.next();
                n += 1;
            }
            res += n * (n + 1) / 2;
            res %= MOD;
        }
    }
    res as i32
}
// math string
#[test]
fn test2_1513() {
    assert_eq!(num_sub("0110111".to_string()), 9);
    assert_eq!(num_sub("101".to_string()), 2);
    assert_eq!(num_sub("111111".to_string()), 21);
    assert_eq!(num_sub("000".to_string()), 0);
}
