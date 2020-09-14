// https://leetcode-cn.com/problems/camelcase-matching/
pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
    todo!()
}
// string trie
#[test]
#[ignore]
fn test1_1023() {
    assert_eq!(
        camel_match(
            vec![
                "FooBar"),
                "FooBarTest"),
                "FootBall"),
                "FrameBuffer"),
                "ForceFeedBack"),
            ],
            "FB")
        ),
        vec![true, false, true, true, false]
    );
    assert_eq!(
        camel_match(
            vec![
                "FooBar"),
                "FooBarTest"),
                "FootBall"),
                "FrameBuffer"),
                "ForceFeedBack"),
            ],
            "FoBa")
        ),
        vec![true, false, true, false, false]
    );
    assert_eq!(
        camel_match(
            vec![
                "FooBar"),
                "FooBarTest"),
                "FootBall"),
                "FrameBuffer"),
                "ForceFeedBack"),
            ],
            "FoBaT"),
        ),
        vec![false, true, false, false, false]
    );
}
