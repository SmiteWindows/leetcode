// https://leetcode-cn.com/problems/camelcase-matching/
pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
    todo!()
}
// string trie
#[test]
#[ignore]
fn test1_1023() {
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
            "FoBaT".to_string()
        ),
        vec![false, true, false, false, false]
    );
}
