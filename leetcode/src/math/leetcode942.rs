// https://leetcode-cn.com/problems/di-string-match/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn di_string_match(s: String) -> Vec<i32> {
    let mut l = 0;
    let mut r = s.len() as i32;
    let mut res = s
        .chars()
        .map(|c| match c {
            'I' => {
                let val = l;
                l += 1;
                val
            }
            'D' => {
                let val = r;
                r -= 1;
                val
            }
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    res.push(l);
    res
}
// math
#[test]
fn test1_942() {
    assert_eq!(di_string_match(String::from("IDID")), vec![0, 4, 1, 3, 2]);
    assert_eq!(di_string_match(String::from("III")), vec![0, 1, 2, 3]);
    assert_eq!(di_string_match(String::from("DDI")), vec![3, 2, 0, 1]);
}
