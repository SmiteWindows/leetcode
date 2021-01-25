// https://leetcode-cn.com/problems/maximum-number-of-balloons/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashMap;
pub fn max_number_of_balloons(text: String) -> i32 {
    let mut text_count: HashMap<char, usize> = HashMap::new();
    let mut ballon_count: HashMap<char, usize> = HashMap::new();
    for c in "balloon".chars() {
        *ballon_count.entry(c).or_default() += 1;
    }
    for c in text.chars() {
        *text_count.entry(c).or_default() += 1;
    }
    let mut min = text.len();
    for (c, v) in ballon_count {
        min = min.min(text_count.get(&c).unwrap_or(&0) / v);
    }
    min as i32
}
// hash_table string
#[test]
fn test2_1189() {
    assert_eq!(max_number_of_balloons("nlaebolko".to_string()), 1);
    assert_eq!(max_number_of_balloons("loonbalxballpoon".to_string()), 2);
    assert_eq!(max_number_of_balloons("leetcode".to_string()), 0);
}
