// https://leetcode-cn.com/problems/tag-validator/
pub fn is_valid(code: String) -> bool {
    todo!()
}
// stack string
#[test]
#[ignore]
fn test2_591() {
    assert_eq!(
        is_valid(String::from(
            "<DIV>This is the first line <![CDATA[<div>]]></DIV>"
        )),
        true
    );
    assert_eq!(
        is_valid(String::from(
            "<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>"
        )),
        true
    );
    assert_eq!(is_valid(String::from("<A>  <B> </A>   </B>")), false);
    assert_eq!(
        is_valid(String::from("<DIV>  div tag is not closed  <DIV>")),
        false
    );
    assert_eq!(is_valid(String::from("<DIV>  unmatched <  </DIV>")), false);
    assert_eq!(
        is_valid(String::from(
            "<DIV> closed tags with invalid tag name  <b>123</b> </DIV>"
        )),
        false
    );
    assert_eq!(
        is_valid(String::from(
            "<DIV> unmatched tags with invalid tag name  </1234567890> and <CDATA[[]]>  </DIV>"
        )),
        false
    );
    assert_eq!(
        is_valid(String::from(
            "<DIV>  unmatched start tag <B>  and unmatched end tag </C>  </DIV>"
        )),
        false
    );
}
