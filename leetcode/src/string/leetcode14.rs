// https://leetcode-cn.com/problems/longest-common-prefix/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }
    let ss = strs
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let n = ss.iter().map(|s| s.len()).min().unwrap();
    let mut prefix = Vec::new();
    for i in 0..n {
        let c = ss[0][i];
        if ss.iter().all(|s| s[i] == c) {
            prefix.push(c);
        } else {
            break;
        }
    }
    prefix.iter().collect()
}
// string
#[test]
fn test1_14() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        longest_common_prefix(vec_string!["flower", "flow", "flight"]),
        "fl".to_string()
    );
    assert_eq!(
        longest_common_prefix(vec_string!["dog", "racecar", "car"]),
        "".to_string()
    );
}
