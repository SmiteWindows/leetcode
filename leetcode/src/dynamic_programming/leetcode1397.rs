// https://leetcode-cn.com/problems/find-all-good-strings/
pub fn find_good_strings(n: i32, s1: String, s2: String, evil: String) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1397() {
    assert_eq!(
        find_good_strings(2, String::from("aa"), String::from("da"), String::from("b")),
        51
    );
    assert_eq!(
        find_good_strings(
            8,
            String::from("leetcode"),
            String::from("leetgoes"),
            String::from("leet")
        ),
        0
    );
    assert_eq!(
        find_good_strings(2, String::from("gx"), String::from("gz"), String::from("x")),
        2
    );
}
