// https://leetcode-cn.com/problems/restore-the-array/
pub fn number_of_arrays(s: String, k: i32) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1416() {
    assert_eq!(number_of_arrays(String::from("1000"), 10000), 1);
    assert_eq!(number_of_arrays(String::from("1000"), 10), 0);
    assert_eq!(number_of_arrays(String::from("1317"), 2000), 8);
    assert_eq!(number_of_arrays(String::from("2020"), 30), 1);
    assert_eq!(number_of_arrays(String::from("1234567890"), 90), 34);
}
