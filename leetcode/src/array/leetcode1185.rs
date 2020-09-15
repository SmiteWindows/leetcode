// https://leetcode-cn.com/problems/day-of-the-week/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn day_of_the_week(day: i32, mut month: i32, mut year: i32) -> String {
    let days = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    if month < 3 {
        month += 12;
        year -= 1;
    }
    let week =
        (day + 2 * month + 3 * (month + 1) / 5 + year + year / 4 - year / 100 + year / 400) % 7 + 1;
    days[week as usize % 7].to_string()
}
// array
#[test]
fn test1_1185() {
    assert_eq!(day_of_the_week(31, 8, 2019), "Saturday".to_string());
    assert_eq!(day_of_the_week(18, 7, 1999), "Sunday".to_string());
    assert_eq!(day_of_the_week(15, 8, 1993), "Sunday".to_string());
}
