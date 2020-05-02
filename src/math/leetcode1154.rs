// https://leetcode.com/problems/day-of-the-year/
pub fn day_of_year(date: String) -> i32 {
    todo!()
}
// math
#[test]
#[ignore]
fn test1_1154() {
    assert_eq!(day_of_year(String::from("2019-01-09")), 9);
    assert_eq!(day_of_year(String::from("2019-02-10")), 41);
    assert_eq!(day_of_year(String::from("2003-03-01")), 60);
    assert_eq!(day_of_year(String::from("2004-03-01")), 61);
}
