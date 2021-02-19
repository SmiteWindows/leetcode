// https://leetcode-cn.com/problems/palindrome-partitioning-iii/
pub fn palindrome_partition(s: String, k: i32) -> i32 {
    todo!()
}
// dynamic_programming array
#[test]
#[ignore]
fn test1_1278() {
    assert_eq!(palindrome_partition("abc".to_string(), 2), 1);
    assert_eq!(palindrome_partition("aabbc".to_string(), 3), 0);
    assert_eq!(palindrome_partition("leetcode".to_string(), 8), 0);
}
