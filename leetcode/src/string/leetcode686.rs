// https://leetcode-cn.com/problems/repeated-string-match/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn repeated_string_match(a: String, b: String) -> i32 {
    let mut s = String::new();
    let an = a.len();
    let bm = b.len();
    if an == 0 || bm == 0 {
        return -1;
    }
    let mut k = bm / an;
    if k * an < bm {
        k += 1;
    }
    for _ in 0..k {
        s += &a;
    }
    if s.contains(&b) {
        return k as i32;
    }
    s += &a;
    if s.contains(&b) {
        return (k + 1) as i32;
    }
    -1
}
// string
#[test]
fn test1_686() {
    assert_eq!(
        repeated_string_match("abcd".to_string(), "cdabcdab".to_string()),
        3
    );
}
