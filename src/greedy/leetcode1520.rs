// https://leetcode.com/problems/maximum-number-of-non-overlapping-substrings/
pub fn max_num_of_substrings(s: String) -> Vec<String> {
    todo!()
}
// greedy
#[test]
#[ignore]
fn test1_1520() {
    assert_eq!(
        max_num_of_substrings("adefaddaccc".to_string()),
        vec!["e".to_string(), "f".to_string(), "ccc".to_string()]
    );
    assert_eq!(
        max_num_of_substrings("abbaccd".to_string()),
        vec!["d".to_string(), "bb".to_string(), "cc".to_string()]
    );
}
