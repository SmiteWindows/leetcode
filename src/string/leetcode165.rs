// https://leetcode.com/problems/compare-version-numbers/
pub fn compare_version(version1: String, version2: String) -> i32 {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_165() {
    assert_eq!(
        compare_version(String::from("0.1"), String::from("1.1")),
        -1
    );
    assert_eq!(compare_version(String::from("1.0.1"), String::from("1")), 1);
    assert_eq!(
        compare_version(String::from("7.5.2.4"), String::from("7.5.3")),
        -1
    );
    assert_eq!(
        compare_version(String::from("1.01"), String::from("1.001")),
        0
    );
    assert_eq!(
        compare_version(String::from("1.0"), String::from("1.0.0")),
        0
    );
}
