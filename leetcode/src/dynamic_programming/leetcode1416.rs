// https://leetcode-cn.com/problems/restore-the-array/
pub fn number_of_arrays(s: String, k: i32) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1416() {
    assert_eq!(number_of_arrays("1000".to_string(), 10000), 1);
    assert_eq!(number_of_arrays("1000".to_string(), 10), 0);
    assert_eq!(number_of_arrays("1317".to_string(), 2000), 8);
    assert_eq!(number_of_arrays("2020".to_string(), 30), 1);
    assert_eq!(number_of_arrays("1234567890".to_string(), 90), 34);
}
