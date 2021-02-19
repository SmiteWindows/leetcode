// https://leetcode-cn.com/problems/my-calendar-ii/
// Runtime: 28 ms
// Memory Usage: 2.6 MB
use std::collections::BTreeMap;
struct MyCalendarTwo {
    overlaps: Vec<(i32, i32)>,
}

impl MyCalendarTwo {
    fn new() -> Self {
        Self { overlaps: vec![] }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let mut triple_booked = MyCalendar::new();
        for &(a, b) in &self.overlaps {
            let a = a.max(start);
            let b = b.min(end);
            if a < b && !triple_booked.book(a, b) {
                return false;
            }
        }
        self.overlaps.push((start, end));
        true
    }
}

struct MyCalendar {
    calendar: BTreeMap<i32, (i32, i32)>,
}

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
 * Your MyCalendarTwo object will be instantiated and called as such:
 * let obj = MyCalendarTwo::new();
 * let ret_1: bool = obj.book(start, end);
 */
// ordered_map
#[test]
fn test1_731() {
    let mut obj = MyCalendarTwo::new();
    assert_eq!(obj.book(10, 20), true);
    assert_eq!(obj.book(50, 60), true);
    assert_eq!(obj.book(10, 40), true);
    assert_eq!(obj.book(5, 15), false);
    assert_eq!(obj.book(5, 10), true);
    assert_eq!(obj.book(25, 55), true);
}
