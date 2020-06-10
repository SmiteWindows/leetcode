// https://leetcode.com/problems/zigzag-conversion/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn convert(s: String, num_rows: i32) -> String {
    let mut res = "".to_string();
    let n = num_rows as usize;
    if n == 1 {
        return s;
    }
    let m = s.len();
    let mut v = vec!["".to_string(); n];
    let mut row = 0;
    let mut direction = true;
    for j in 0..m {
        v[row] += &s[j..=j];
        if row == 0 {
            direction = true;
            row += 1;
        } else if row == n - 1 {
            direction = false;
            row -= 1;
        } else {
            if direction {
                row += 1;
            } else {
                row -= 1;
            }
        }
    }
    for t in v {
        res += &t;
    }
    res
}
// string
#[test]
fn test1_6() {
    assert_eq!(
        convert(String::from("PAYPALISHIRING"), 3),
        String::from("PAHNAPLSIIGYIR")
    );
    assert_eq!(
        convert(String::from("PAYPALISHIRING"), 4),
        String::from("PINALSIGYAHRPI")
    );
}
