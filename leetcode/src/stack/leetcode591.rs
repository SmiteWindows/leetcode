// https://leetcode-cn.com/problems/tag-validator/
pub fn is_valid(code: String) -> bool {
    todo!()
}
// stack string
#[test]
#[ignore]
fn test1_591() {
    assert_eq!(
        is_valid("<DIV>This is the first line <![CDATA[<div>]]></DIV>".to_string()),
        true
    );
    assert_eq!(
        is_valid("<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>".to_string()),
        true
    );
    assert_eq!(is_valid("<A>  <B> </A>   </B>".to_string()), false);
    assert_eq!(
        is_valid("<DIV>  div tag is not closed  <DIV>".to_string()),
        false
    );
    assert_eq!(is_valid("<DIV>  unmatched <  </DIV>".to_string()), false);
    assert_eq!(
        is_valid("<DIV> closed tags with invalid tag name  <b>123</b> </DIV>".to_string()),
        false
    );
    assert_eq!(
        is_valid(
            "<DIV> unmatched tags with invalid tag name  </1234567890> and <CDATA[[]]>  </DIV>"
                .to_string()
        ),
        false
    );
    assert_eq!(
        is_valid("<DIV>  unmatched start tag <B>  and unmatched end tag </C>  </DIV>".to_string()),
        false
    );
}
