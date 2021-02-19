// https://leetcode-cn.com/problems/jewels-and-stones/
pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
    s.chars().into_iter().filter(|&c| j.contains(c)).count() as i32
}
// Runtime: 0 ms
// Memory Usage: 1.9 MB
// ✔
// hash_table
#[test]
fn test1_771() {
    assert_eq!(
        num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
        3
    );
    assert_eq!(num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
}
