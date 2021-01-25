// https://leetcode-cn.com/problems/summary-ranges/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::fmt;
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut r: Option<Range> = None; // ?
    let mut res = vec![];
    for x in nums {
        if let Some(prev) = r {
            if prev.end + 1 == x {
                r = Some(Range {
                    start: prev.start,
                    end: x,
                });
                continue;
            } else {
                res.push(format!("{}", prev));
                r = Some(Range { start: x, end: x });
            }
        } else {
            r = Some(Range { start: x, end: x })
        }
    }
    if let Some(last) = r {
        res.push(format!("{}", last));
    }
    res
}

struct Range {
    start: i32,
    end: i32,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start == self.end {
            write!(f, "{}", self.start)
        } else {
            write!(f, "{}->{}", self.start, self.end)
        }
    }
}
// array
#[test]
fn test1_228() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        summary_ranges(vec![0, 1, 2, 4, 5, 7]),
        vec_string!["0->2", "4->5", "7"]
    );
    assert_eq!(
        summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
        vec_string!["0", "2->4", "6", "8->9"]
    );
}
