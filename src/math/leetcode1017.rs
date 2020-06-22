// https://leetcode.com/problems/convert-to-base-2/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn base_neg2(n: i32) -> String {
    let mut n = n;
    if n == 0 {
        return "0".to_string();
    }
    let mut res = vec![];
    while n != 0 {
        res.push((b'0' + (n & 1) as u8) as char);
        n = -(n >> 1);
    }
    res.reverse();
    res.into_iter().collect()
}
// math
#[test]
fn test1_1017() {
    assert_eq!(base_neg2(2), String::from("110"));
    assert_eq!(base_neg2(3), String::from("111"));
    assert_eq!(base_neg2(4), String::from("100"));
}
