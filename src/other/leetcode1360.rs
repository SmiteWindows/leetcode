// https://leetcode.com/problems/number-of-days-between-two-dates/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn days_between_dates(date1: String, date2: String) -> i32 {
    let (y1, m1, d1) = parse(date1);
    let (y2, m2, d2) = parse(date2);
    let mut s1 = 0;
    let mut s2 = 0;
    for i in 1971..y1 {
        s1 += if is_leap(i) { 366 } else { 365 };
    }
    for i in 1971..y2 {
        s2 += if is_leap(i) { 366 } else { 365 };
    }
    s1 += day_of_year(y1, m1, d1);
    s2 += day_of_year(y2, m2, d2);
    (s1 as i32 - s2 as i32).abs()
}

fn parse(date: String) -> (usize, usize, usize) {
    let a = date.split_terminator('-').collect::<Vec<_>>();
    let year = a[0].parse::<usize>().expect("exist");
    let month = a[1].parse::<usize>().expect("exist");
    let day = a[2].parse::<usize>().expect("exist");
    (year, month, day)
}

fn is_leap(year: usize) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn day_of_year(year: usize, month: usize, day: usize) -> usize {
    let mut days = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut sum = 0;
    if is_leap(year) {
        days[1] += 1;
    }
    for d in days.iter().take(month - 1) {
        sum += d;
    }
    sum += day;
    sum
}
#[test]
fn test1360() {
    assert_eq!(
        days_between_dates("2019-06-29".to_string(), "2019-06-30".to_string()),
        1
    );
    assert_eq!(
        days_between_dates("2020-01-15".to_string(), "2019-12-31".to_string()),
        15
    );
}
