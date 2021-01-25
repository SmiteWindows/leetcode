// https://leetcode-cn.com/problems/valid-square/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
use crate::d;
use std::collections::HashSet;
pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
    let mut hs = HashSet::new();
    let v = vec![p1, p2, p3, p4];
    for i in 0..4 {
        for j in i + 1..4 {
            hs.insert(d!(v[i], v[j]));
        }
    }
    hs.len() == 2 && !hs.contains(&0)
}

#[macro_export]
macro_rules! d {
    ($a:expr, $b:expr) => {
        ($a[0] - $b[0]) * ($a[0] - $b[0]) + ($a[1] - $b[1]) * ($a[1] - $b[1])
    };
}
// math
#[test]
fn test1_593() {
    assert_eq!(
        valid_square(vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1]),
        true
    );
}
