// https://leetcode-cn.com/problems/find-and-replace-in-string/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn find_replace_string(
    s: String,
    indexes: Vec<i32>,
    sources: Vec<String>,
    targets: Vec<String>,
) -> String {
    let mut s = s;
    let mut v = Vec::new();
    for (i, &index) in indexes.iter().enumerate() {
        v.push((index as usize, i));
    }
    v.sort_unstable();
    for (i, j) in v.into_iter().rev() {
        let source = &sources[j];
        let m = source.len();
        let target = &targets[j];
        if i + m <= s.len() && &s[i..i + m] == source {
            s.replace_range(i..i + m, target);
        }
    }
    s
}
// string
#[test]
fn test1_833() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        find_replace_string(
            "abcd".to_string(),
            vec![0, 2],
            vec_string!["a", "cd"],
            vec_string!["eee", "ffff"]
        ),
        "eeebffff".to_string()
    );
    assert_eq!(
        find_replace_string(
            "abcd".to_string(),
            vec![0, 2],
            vec_string!["ab", "ec"],
            vec_string!["eee", "ffff"]
        ),
        "eeecd".to_string()
    );
}
