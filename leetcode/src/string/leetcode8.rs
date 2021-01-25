// https://leetcode-cn.com/problems/string-to-integer-atoi/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn my_atoi(str: String) -> i32 {
    let mut start = str.trim_start();
    let mut res = 0_i32;
    let mut positive = true;
    if start.len() > 1 {
        let c = &start[0..1];
        match c {
            "+" => {
                start = &start[1..];
            }
            "-" => {
                start = &start[1..];
                positive = false;
            }
            _ => {
                if let Some(c) = c.chars().next() {
                    if !('0'..='9').contains(&c) {
                        return 0;
                    }
                }
            }
        }
    }
    for c in start.chars() {
        if ('0'..='9').contains(&c) {
            res = match res.checked_mul(10) {
                None => {
                    return overflow(positive);
                }
                Some(val) => val,
            };
            res = match res.checked_add((c as u8 - b'0') as i32) {
                None => {
                    return overflow(positive);
                }
                Some(val) => val,
            };
        } else {
            break;
        }
    }
    if !positive {
        res = match res.checked_mul(-1) {
            None => {
                return overflow(positive);
            }
            Some(val) => val,
        };
    }
    res
}

fn overflow(positive: bool) -> i32 {
    if positive {
        i32::MAX
    } else {
        i32::MIN
    }
}
// math string
#[test]
fn test1_8() {
    assert_eq!(my_atoi("42".to_string()), 42);
    assert_eq!(my_atoi("   -42".to_string()), -42);
    assert_eq!(my_atoi("4193 with words".to_string()), 4193);
    assert_eq!(my_atoi("words and 987".to_string()), 0);
    assert_eq!(my_atoi("-91283472332".to_string()), -2147483648);
}
