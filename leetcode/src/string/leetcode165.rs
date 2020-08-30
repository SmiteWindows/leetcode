// https://leetcode-cn.com/problems/compare-version-numbers/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn compare_version(version1: String, version2: String) -> i32 {
    let version1 = version1
        .split_terminator('.')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let version2 = version2
        .split_terminator('.')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let n1 = version1.len();
    let n2 = version2.len();
    for i in 0..n1.max(n2) {
        let x1 = version1.get(i).unwrap_or(&0);
        let x2 = version2.get(i).unwrap_or(&0);
        if x1 > x2 {
            return 1;
        }
        if x1 < x2 {
            return -1;
        }
    }
    0
}
// string
#[test]
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
