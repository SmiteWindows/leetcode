// https://leetcode.com/problems/my-calendar-i/
// Runtime: 24 ms
// Memory Usage: 2.4 MB
use std::collections::BTreeMap;
struct MyCalendar {
    calendar: BTreeMap<i32, (i32, i32)>,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        Self {
            calendar: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let center = start + end;
        if let Some((_, &(_, last_end))) = self.calendar.range(..=center).next_back() {
            if start < last_end {
                return false;
            }
        }
        if let Some((_, &(first_start, _))) = self.calendar.range(center..).next() {
            if first_start < end {
                return false;
            }
        }
        self.calendar.insert(center, (start, end));
        true
    }
}
/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */
// array
#[test]
fn test1_729() {
    let mut obj = MyCalendar::new();
    assert_eq!(obj.book(10, 20), true);
    assert_eq!(obj.book(15, 25), false);
    assert_eq!(obj.book(20, 30), true);
}
