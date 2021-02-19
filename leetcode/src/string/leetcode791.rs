// https://leetcode-cn.com/problems/custom-sort-string/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn custom_sort_string(s: String, t: String) -> String {
    use std::collections::HashMap;
    let mut hm: HashMap<char, usize> = HashMap::new();
    for (i, c) in s.char_indices() {
        *hm.entry(c).or_default() = i;
    }
    let mut t = t.chars().collect::<Vec<_>>();
    t.sort_unstable_by_key(|c| hm.get(c).unwrap_or(&26));
    t.iter().collect()
}
// string
#[test]
fn test1_791() {
    assert_eq!(
        custom_sort_string("cba".to_string(), "abcd".to_string()),
        "cbad".to_string()
    );
}
