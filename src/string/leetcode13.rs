// https://leetcode.com/problems/roman-to-integer/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashMap;
pub fn roman_to_int(s: String) -> i32 {
    let map = vec![
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();
    let mut sum = 0;
    let mut last = 0;
    for c in s.chars() {
        if let Some(&v) = map.get(&c) {
            if v > last {
                sum += v - last - last;
            } else {
                sum += v
            }
            last = v;
        }
    }
    sum
}
// math string
#[test]
fn test1_13() {
    assert_eq!(roman_to_int(String::from("III")), 3);
    assert_eq!(roman_to_int(String::from("IV")), 4);
    assert_eq!(roman_to_int(String::from("IX")), 9);
    assert_eq!(roman_to_int(String::from("LVIII")), 58);
    assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
}
