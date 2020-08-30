// https://leetcode-cn.com/problems/count-and-say/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn count_and_say(n: i32) -> String {
    match n {
        1 => "1".to_string(),
        2..=30 => next(count_and_say(n - 1)),
        _ => "".to_string(),
    }
}

struct Pair {
    digit: char,
    count: usize,
}

fn next(nums: String) -> String {
    let mut s = String::from("");
    let mut prev: Option<Pair> = None;
    for c in nums.chars() {
        if let Some(prev_pair) = prev {
            if prev_pair.digit == c {
                prev = Some(Pair {
                    digit: c,
                    count: prev_pair.count + 1,
                });
            } else {
                s.push_str(&prev_pair.count.to_string());
                s.push_str(&prev_pair.digit.to_string());
                prev = Some(Pair { digit: c, count: 1 });
            }
        } else {
            prev = Some(Pair { digit: c, count: 1 });
        }
    }
    if let Some(prev_pair) = prev {
        s.push_str(&prev_pair.count.to_string());
        s.push_str(&prev_pair.digit.to_string());
    }
    s
}
// string
#[test]
fn test1_38() {
    assert_eq!(count_and_say(1), String::from("1"));
    assert_eq!(count_and_say(4), String::from("1211"));
}
