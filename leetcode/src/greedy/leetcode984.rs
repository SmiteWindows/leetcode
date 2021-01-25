// https://leetcode-cn.com/problems/string-without-aaa-or-bbb/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::cmp::Ordering::{Equal, Greater, Less};
pub fn str_without3a3b(mut a: i32, mut b: i32) -> String {
    let mut res = "".to_string();
    match a.cmp(&b) {
        Less => {
            while b > 0 {
                res += "b";
                b -= 1;
                if b > a {
                    res += "b";
                    b -= 1;
                }
                if a > 0 {
                    res += "a";
                    a -= 1;
                }
            }
        }
        Equal => {
            for _ in 0..a {
                res += "ab";
            }
        }
        Greater => {
            while a > 0 {
                res += "a";
                a -= 1;
                if a > b {
                    res += "a";
                    a -= 1;
                }
                if b > 0 {
                    res += "b";
                    b -= 1;
                }
            }
        }
    }
    res
}
// greedy
#[test]
fn test1_984() {
    assert_eq!(str_without3a3b(1, 2), "bab".to_string());
    assert_eq!(str_without3a3b(4, 1), "aabaa".to_string());
}
