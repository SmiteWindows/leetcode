// https://leetcode.com/problems/count-largest-group/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::collections::HashMap;
pub fn count_largest_group(n: i32) -> i32 {
    let mut hm: HashMap<i32, usize> = HashMap::new();
    let mut max = 0;
    for i in 1..=n {
        let mut sum = 0;
        let mut n = i;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }
        let count = hm.entry(sum).or_default();
        *count += 1;
        max = max.max(*count);
    }
    hm.values().filter(|&&v| v == max).count() as i32
}
// array
#[test]
fn test1_1399() {
    assert_eq!(count_largest_group(13), 4);
    assert_eq!(count_largest_group(2), 2);
    assert_eq!(count_largest_group(15), 6);
    assert_eq!(count_largest_group(24), 5);
}
