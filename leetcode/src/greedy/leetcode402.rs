// https://leetcode-cn.com/problems/remove-k-digits/
pub fn remove_kdigits(num: String, k: i32) -> String {
    todo!()
}
// stack greedy
#[test]
#[ignore]
fn test2_402() {
    assert_eq!(remove_kdigits("1432219".to_string(), 3), "1219".to_string());
    assert_eq!(remove_kdigits("10200".to_string(), 1), "200".to_string());
    assert_eq!(remove_kdigits("10".to_string(), 2), "0".to_string());
}
