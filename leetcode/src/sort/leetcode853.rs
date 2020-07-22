// https://leetcode.com/problems/car-fleet/
// Runtime: 8 ms
// Memory Usage: 2.4 MB
use std::collections::BTreeMap;
pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut btm = BTreeMap::new();
    let n = position.len();
    for i in 0..n {
        btm.insert(
            target - position[i],
            (target - position[i]) as f64 / speed[i] as f64,
        );
    }
    let mut res = 0;
    let mut cur = 0.0;
    for (_, t) in btm {
        if t > cur {
            res += 1;
            cur = t;
        }
    }
    res
}
// sort
#[test]
fn test1_853() {
    assert_eq!(car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]), 3);
}
