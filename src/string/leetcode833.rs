// https://leetcode.com/problems/find-and-replace-in-string/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn find_replace_string(
    s: String,
    indexes: Vec<i32>,
    sources: Vec<String>,
    targets: Vec<String>,
) -> String {
    let mut s = s;
    let n = indexes.len();
    let mut v = vec![];
    for i in 0..n {
        v.push((indexes[i] as usize, i));
    }
    v.sort_unstable();
    for (i, j) in v.into_iter().rev() {
        let source = &sources[j];
        let m = source.len();
        let target = &targets[j];
        if i + m <= s.len() && &s[i..i + m] == source {
            s.replace_range(i..i + m, &target);
        }
    }
    s
}
// string
#[test]
fn test1_833() {
    assert_eq!(
        find_replace_string(
            String::from("abcd"),
            vec![0, 2],
            vec![String::from("a"), String::from("cd")],
            vec![String::from("eee"), String::from("ffff")]
        ),
        String::from("eeebffff")
    );
    assert_eq!(
        find_replace_string(
            String::from("abcd"),
            vec![0, 2],
            vec![String::from("ab"), String::from("ec")],
            vec![String::from("eee"), String::from("ffff")]
        ),
        String::from("eeecd")
    );
}
