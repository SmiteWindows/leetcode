// https://leetcode.com/problems/smallest-good-base/
pub fn smallest_good_base(n: String) -> String {
    todo!()
}
// math binary_search
#[test]
#[ignore]
fn test2_483() {
    assert_eq!(smallest_good_base(String::from("13")), String::from("3"));
    assert_eq!(smallest_good_base(String::from("4681")), String::from("8"));
    assert_eq!(
        smallest_good_base(String::from("1000000000000000000")),
        String::from("999999999999999999")
    );
}
