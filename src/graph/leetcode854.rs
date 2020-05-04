// https://leetcode.com/problems/k-similar-strings/
pub fn k_similarity(a: String, b: String) -> i32 {
    todo!()
}
// graph breadth_first_search
#[test]
#[ignore]
fn test1_854() {
    assert_eq!(k_similarity(String::from("ab"), String::from("ba")), 1);
    assert_eq!(k_similarity(String::from("abc"), String::from("bca")), 2);
    assert_eq!(k_similarity(String::from("abac"), String::from("baca")), 2);
    assert_eq!(k_similarity(String::from("aabc"), String::from("abca")), 2);
}
