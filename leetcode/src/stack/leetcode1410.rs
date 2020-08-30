// https://leetcode-cn.com/problems/html-entity-parser/
pub fn entity_parser(text: String) -> String {
    todo!()
}
// string stack
#[test]
#[ignore]
fn test1_1410() {
    assert_eq!(
        entity_parser(String::from(
            "&amp; is an HTML entity but &ambassador; is not."
        )),
        String::from("& is an HTML entity but &ambassador; is not.")
    );
    assert_eq!(
        entity_parser(String::from("and I quote: &quot;...&quot;")),
        String::from("and I quote: \"...\"")
    );
    assert_eq!(
        entity_parser(String::from("Stay home! Practice on Leetcode :)")),
        String::from("Stay home! Practice on Leetcode :)")
    );
    assert_eq!(
        entity_parser(String::from("x &gt; y &amp;&amp; x &lt; y is always false")),
        String::from("x > y && x < y is always false")
    );
    assert_eq!(
        entity_parser(String::from("leetcode.com&frasl;problemset&frasl;all")),
        String::from("leetcode.com/problemset/all")
    );
}
