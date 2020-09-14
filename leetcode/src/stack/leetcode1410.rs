// https://leetcode-cn.com/problems/html-entity-parser/
pub fn entity_parser(text: String) -> String {
    todo!()
}
// string stack
#[test]
#[ignore]
fn test1_1410() {
    assert_eq!(
        entity_parser(
            "&amp; is an HTML entity but &ambassador; is not."
        .to_string()),
        "& is an HTML entity but &ambassador; is not.")
    );
    assert_eq!(
        entity_parser("and I quote: &quot;...&quot;".to_string()),
        "and I quote: \"...\"".to_string()
    );
    assert_eq!(
        entity_parser("Stay home! Practice on Leetcode :)".to_string()),
        "Stay home! Practice on Leetcode :)".to_string()
    );
    assert_eq!(
        entity_parser("x &gt; y &amp;&amp; x &lt; y is always false".to_string()),
        "x > y && x < y is always false".to_string()
    );
    assert_eq!(
        entity_parser("leetcode.com&frasl;problemset&frasl;all".to_string()),
        "leetcode.com/problemset/all".to_string()
    );
}
