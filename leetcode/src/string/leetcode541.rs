// https://leetcode-cn.com/problems/reverse-string-ii/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn reverse_str(s: String, k: i32) -> String {
    let k = k as usize;
    let mut s = s.chars().collect::<Vec<_>>();
    let n = s.len();
    let mut i = 0;
    while i * 2 * k < n {
        let left = (i + 1) * 2 * k;
        if left < n {
            let ss = &mut s[i * 2 * k..left];
            rev_half(ss, k);
        } else {
            let ss = &mut s[i * 2 * k..n];
            rev_half(ss, k);
        }
        i += 1;
    }
    s.iter().collect()
}

fn rev_half(s: &mut [char], k: usize) -> &[char] {
    if s.len() <= k {
        s.reverse();
    } else {
        let half = &mut s[..k];
        half.reverse();
    }
    s
}
// string
#[test]
fn test1_541() {
    assert_eq!(reverse_str("abcdefg".to_string(), 2), "bacdfeg".to_string());
}
