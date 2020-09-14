// https://leetcode-cn.com/problems/html-entity-parser/
// Runtime: 8 ms
// Memory Usage: 2.5 MB
pub fn entity_parser(text: String) -> String {
    text.replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&frasl;", "/")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}
// string stack
#[test]
fn test1_1410() {
    assert_eq!(
        entity_parser(
            "&amp; is an HTML entity but &ambassador; is not."
        )),
        "& is an HTML entity but &ambassador; is not.")
    );
    assert_eq!(
        entity_parser("and I quote: &quot;...&quot;")),
        "and I quote: \"...\"")
    );
    assert_eq!(
        entity_parser("Stay home! Practice on Leetcode :)")),
        "Stay home! Practice on Leetcode :)")
    );
    assert_eq!(
        entity_parser("x &gt; y &amp;&amp; x &lt; y is always false")),
        "x > y && x < y is always false")
    );
    assert_eq!(
        entity_parser("leetcode.com&frasl;problemset&frasl;all")),
        "leetcode.com/problemset/all")
    );
}
