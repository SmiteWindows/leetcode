// https://leetcode.com/problems/smallest-string-with-swaps/
pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
    todo!()
}
// union_find array
#[test]
#[ignore]
fn test1_1202() {
    assert_eq!(
        smallest_string_with_swaps(String::from("dcab"), vec![vec![0, 3], vec![1, 2]]),
        String::from("bacd")
    );
    assert_eq!(
        smallest_string_with_swaps(String::from("dcab"), vec![vec![0, 3], vec![1, 2]]),
        String::from("abcd")
    );
    assert_eq!(
        smallest_string_with_swaps(String::from("cba"), vec![vec![0, 1], vec![1, 2]]),
        String::from("abc")
    );
}
