// https://leetcode.com/problems/masking-personal-information/
pub fn mask_pii(s: String) -> String {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_831() {
    assert_eq!(
        mask_pii(String::from("LeetCode@LeetCode.com")),
        String::from("l*****e@leetcode.com")
    );
    assert_eq!(
        mask_pii(String::from("AB@qq.com")),
        String::from("a*****b@qq.com")
    );
    assert_eq!(
        mask_pii(String::from("1(234)567-890")),
        String::from("***-***-7890")
    );
    assert_eq!(
        mask_pii(String::from("86-(10)12345678")),
        String::from("+**-***-***-5678")
    );
}
