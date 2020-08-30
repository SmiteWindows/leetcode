// https://leetcode-cn.com/problems/happy-number/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashSet;
pub fn is_happy(n: i32) -> bool {
    fn get_next(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            let d = n % 10;
            n /= 10;
            sum += d * d;
        }
        sum
    }

    let mut cycle_members = HashSet::new();
    cycle_members.insert(4);
    cycle_members.insert(16);
    cycle_members.insert(37);
    cycle_members.insert(58);
    cycle_members.insert(89);
    cycle_members.insert(145);
    cycle_members.insert(42);
    cycle_members.insert(20);
    let mut n = n;
    while n != 1 && !cycle_members.contains(&n) {
        n = get_next(n);
    }
    n == 1
}
// math hash_table
#[test]
fn test2_202() {
    assert_eq!(is_happy(19), true);
}
