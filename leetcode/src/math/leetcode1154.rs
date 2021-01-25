// https://leetcode-cn.com/problems/day-of-the-year/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn day_of_year(date: String) -> i32 {
    let a = date.split_terminator('-').collect::<Vec<_>>();
    let year = a[0].parse::<usize>().unwrap();
    let month = a[1].parse::<usize>().unwrap();
    let day = a[2].parse::<usize>().unwrap();
    let mut days = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut sum = 0;
    if is_leap(year) {
        days[1] += 1;
    }
    for d in days.iter().take(month - 1) {
        sum += d;
    }
    sum += day;
    sum as i32
}

fn is_leap(year: usize) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
// math
#[test]
fn test1_1154() {
    assert_eq!(day_of_year("2019-01-09".to_string()), 9);
    assert_eq!(day_of_year("2019-02-10".to_string()), 41);
    assert_eq!(day_of_year("2003-03-01".to_string()), 60);
    assert_eq!(day_of_year("2004-03-01".to_string()), 61);
}
