// https://leetcode-cn.com/problems/find-all-good-strings/
pub fn find_good_strings(n: i32, s1: String, s2: String, evil: String) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1397() {
    assert_eq!(
        find_good_strings(2, "aa".to_string(), "da".to_string(), "b".to_string()),
        51
    );
    assert_eq!(
        find_good_strings(
            8,
            "leetcode".to_string(),
            "leetgoes".to_string(),
            "leet".to_string()
        ),
        0
    );
    assert_eq!(
        find_good_strings(2, "gx".to_string(), "gz".to_string(), "x".to_string()),
        2
    );
}
