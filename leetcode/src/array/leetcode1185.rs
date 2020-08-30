// https://leetcode-cn.com/problems/day-of-the-week/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
    let mut month = month;
    let mut year = year;
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
    assert_eq!(day_of_the_week(31, 8, 2019), String::from("Saturday"));
    assert_eq!(day_of_the_week(18, 7, 1999), String::from("Sunday"));
    assert_eq!(day_of_the_week(15, 8, 1993), String::from("Sunday"));
}
