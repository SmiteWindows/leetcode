// https://leetcode-cn.com/problems/camelcase-matching/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
    queries
        .into_iter()
        .map(|query| query_match(query.chars().collect(), pattern.chars().collect()))
        .collect()
}

fn query_match(query: Vec<char>, pattern: Vec<char>) -> bool {
    let mut i = 0;
    let n = pattern.len();
    for &q in &query {
        if i < n && q == pattern[i] {
            i += 1;
        } else if q.is_uppercase() {
            return false;
        }
    }
    i == n
}
// string trie
#[test]
fn test2_1023() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        camel_match(
            vec_string![
                "FooBar",
                "FooBarTest",
                "FootBall",
                "FrameBuffer",
                "ForceFeedBack"
            ],
            "FB".to_string()
        ),
        vec![true, false, true, true, false]
    );
    assert_eq!(
        camel_match(
            vec_string![
                "FooBar",
                "FooBarTest",
                "FootBall",
                "FrameBuffer",
                "ForceFeedBack"
            ],
            "FoBa".to_string()
        ),
        vec![true, false, true, false, false]
    );
    assert_eq!(
        camel_match(
            vec_string![
                "FooBar",
                "FooBarTest",
                "FootBall",
                "FrameBuffer",
                "ForceFeedBack"
            ],
            "FoBaT".to_string(),
        ),
        vec![false, true, false, false, false]
    );
}
