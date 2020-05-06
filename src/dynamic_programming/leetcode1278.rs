// https://leetcode.com/problems/palindrome-partitioning-iii/
pub fn palindrome_partition(s: String, k: i32) -> i32 {
    todo!()
}
// dynamic_programming array
#[test]
#[ignore]
fn test1_1278() {
    assert_eq!(palindrome_partition(String::from("abc"), 2), 1);
    assert_eq!(palindrome_partition(String::from("aabbc"), 3), 0);
    assert_eq!(palindrome_partition(String::from("leetcode"), 8), 0);
}
