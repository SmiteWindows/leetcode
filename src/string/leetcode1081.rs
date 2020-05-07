// https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/
pub fn smallest_subsequence(text: String) -> String {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_1081() {
    assert_eq!(
        smallest_subsequence(String::from("cdadabcc")),
        String::from("adbc")
    );
    assert_eq!(
        smallest_subsequence(String::from("abcd")),
        String::from("abcd")
    );
    assert_eq!(
        smallest_subsequence(String::from("ecbacba")),
        String::from("eacb")
    );
    assert_eq!(
        smallest_subsequence(String::from("leetcode")),
        String::from("letcod")
    );
}
