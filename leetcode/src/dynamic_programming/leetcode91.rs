// https://leetcode-cn.com/problems/decode-ways/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn num_decodings(s: String) -> i32 {
    let s = s.bytes().map(|b| b - b'0').collect::<Vec<_>>();
    let n = s.len();
    let mut a = vec![0; n + 1];
    if n == 0 {
        return 0;
    }
    a[0] = 1;
    a[1] = if s[0] > 0 { 1 } else { 0 };
    for i in 1..n {
        let first = s[i];
        let second = s[i - 1] * 10 + s[i];
        if (1..=9).contains(&first) {
            a[i + 1] += a[i];
        }
        if (10..=26).contains(&second) {
            a[i + 1] += a[i - 1];
        }
    }
    a[n]
}
// dynamic_programming string
#[test]
fn test1_91() {
    assert_eq!(num_decodings("12".to_string()), 2);
    assert_eq!(num_decodings("226".to_string()), 3);
}
