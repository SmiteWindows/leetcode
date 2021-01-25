// https://leetcode-cn.com/problems/convert-to-base-2/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn base_neg2(mut n: i32) -> String {
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
    assert_eq!(base_neg2(2), "110".to_string());
    assert_eq!(base_neg2(3), "111".to_string());
    assert_eq!(base_neg2(4), "100".to_string());
}
