// https://leetcode-cn.com/problems/largest-time-for-given-digits/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
use std::{cmp::Ordering, fmt};
pub fn largest_time_from_digits(a: Vec<i32>) -> String {
    let mut a = a;
    let mut max = None;
    backtrack(&mut a, 0, &mut max);
    if let Some(max_time) = max {
        max_time.to_string()
    } else {
        "".to_string()
    }
}

fn backtrack(a: &mut Vec<i32>, index: usize, max: &mut Option<Time>) {
    let n = a.len();
    if index == n {
        let time = Time::from_digits(a);
        if time.is_valid() {
            if let Some(max_time) = max {
                if time > *max_time {
                    *max = Some(time)
                }
            } else {
                *max = Some(time)
            }
        }
    } else {
        for i in index..n {
            a.swap(index, i);
            backtrack(a, index + 1, max);
            a.swap(index, i);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Time {
    hour: i32,
    minute: i32,
}

impl Time {
    fn new(hour: i32, minute: i32) -> Self {
        Self { hour, minute }
    }

    fn is_valid(&self) -> bool {
        self.hour < 24 && self.minute < 60
    }

    fn from_digits(a: &[i32]) -> Self {
        Self::new(a[0] * 10 + a[1], a[2] * 10 + a[3])
    }

    fn to_minutes(&self) -> i32 {
        self.hour * 60 + self.minute
    }

    fn to_digits(&self) -> Vec<i32> {
        vec![
            self.hour / 10,
            self.hour % 10,
            self.minute / 10,
            self.minute % 10,
        ]
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_digits();
        write!(f, "{}{}:{}{}", a[0], a[1], a[2], a[3])
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.to_minutes().cmp(&other.to_minutes()))
    }
}
// math
#[test]
fn test1_949() {
    assert_eq!(
        largest_time_from_digits(vec![1, 2, 3, 4]),
        String::from("23:41")
    );
    assert_eq!(largest_time_from_digits(vec![5, 5, 5, 5]), String::from(""));
}
