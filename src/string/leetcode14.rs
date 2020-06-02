// https://leetcode.com/problems/longest-common-prefix/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::from("");
    }
    let ss = strs
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let n = ss.iter().map(|s| s.len()).min().unwrap();
    let mut prefix = vec![];
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
    assert_eq!(
        longest_common_prefix(vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight")
        ]),
        String::from("fl")
    );
    assert_eq!(
        longest_common_prefix(vec![
            String::from("dog"),
            String::from("racecar"),
            String::from("car")
        ]),
        String::from("")
    );
}
