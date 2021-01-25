// https://leetcode-cn.com/problems/string-compression-ii/
pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
    todo!()
}
// string dynamic_programming
#[test]
#[ignore]
fn test2_1531() {
    assert_eq!(
        get_length_of_optimal_compression("aaabcccd".to_string(), 2),
        4
    );
    assert_eq!(
        get_length_of_optimal_compression("aabbaa".to_string(), 2),
        2
    );
    assert_eq!(
        get_length_of_optimal_compression("aaaaaaaaaaa".to_string(), 0),
        3
    );
}
