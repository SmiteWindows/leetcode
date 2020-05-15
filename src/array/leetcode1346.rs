// https://leetcode.com/problems/check-if-n-and-its-double-exist/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashSet;
pub fn check_if_exist(arr: Vec<i32>) -> bool {
    let mut hs = HashSet::new();
    let mut zero = 0;
    for &x in &arr {
        if x == 0 {
            if zero > 0 {
                return true;
            } else {
                zero += 1;
            }
        } else {
            hs.insert(x);
        }
    }
    for x in arr {
        if hs.contains(&(2 * x)) {
            return true;
        }
    }
    false
}
// array
#[test]
fn test1_1346() {
    assert_eq!(check_if_exist(vec![10, 2, 5, 3]), true);
    assert_eq!(check_if_exist(vec![7, 1, 14, 11]), true);
    assert_eq!(check_if_exist(vec![3, 1, 7, 11]), false);
}
