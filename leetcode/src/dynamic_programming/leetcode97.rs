// https://leetcode.com/problems/interleaving-string/
pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    todo!()
}
// dynamic_programming string
#[test]
#[ignore]
fn test1_97() {
    assert_eq!(
        is_interleave(
            String::from("aabcc"),
            String::from("dbbca"),
            String::from("aadbbcbcac")
        ),
        true
    );
    assert_eq!(
        is_interleave(
            String::from("aabcc"),
            String::from("dbbca"),
            String::from("aadbbbaccc")
        ),
        false
    );
}
