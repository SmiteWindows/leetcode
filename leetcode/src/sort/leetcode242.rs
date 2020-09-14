// https://leetcode-cn.com/problems/valid-anagram/
// Runtime: 4 ms
// Memory Usage: 2.5 MB
pub fn is_anagram(s: String, t: String) -> bool {
    let mut s = s.chars().collect::<Vec<_>>();
    let mut t = t.chars().collect::<Vec<_>>();
    s.sort_unstable();
    t.sort_unstable();
    s == t
}
// sort hash_table
#[test]
fn test1_242() {
    assert_eq!(
        is_anagram("anagram".to_string(), "nagaram".to_string()),
        true
    );
    assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false);
}
