// https://leetcode-cn.com/problems/special-binary-string/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::cmp::Reverse;
pub fn make_largest_special(s: String) -> String {
    let mut count = 0;
    let mut v = vec![];
    let mut ss = "".to_string();
    for c in s.chars() {
        ss.push(c);
        if c == '1' {
            count += 1;
        } else {
            count -= 1;
        }
        if count == 0 {
            let n = ss.len();
            let t = format!("1{}0", make_largest_special(ss[1..n - 1].to_string()));
            v.push(t);
            ss = "".to_string();
        }
    }
    v.sort_unstable_by_key(|s| Reverse(s.to_string()));
    v.join("")
}
// string recursion
#[test]
fn test2_761() {
    assert_eq!(
        make_largest_special("11011000".to_string()),
        "11100100".to_string()
    );
}
