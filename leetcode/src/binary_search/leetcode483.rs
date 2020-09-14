// https://leetcode-cn.com/problems/smallest-good-base/
pub fn smallest_good_base(n: String) -> String {
    todo!()
}
// math binary_search
#[test]
#[ignore]
fn test1_483() {
    assert_eq!(smallest_good_base("13".to_string()), "3".to_string());
    assert_eq!(smallest_good_base("4681".to_string()), "8".to_string());
    assert_eq!(
        smallest_good_base("1000000000000000000".to_string()),
        "999999999999999999".to_string()
    );
}
