// https://leetcode-cn.com/problems/find-a-value-of-a-mysterious-function-closest-to-target/
// Runtime: 124 ms
// Memory Usage: 3.1 MB
use std::collections::HashSet;
pub fn closest_to_target(arr: Vec<i32>, target: i32) -> i32 {
    let mut prev: HashSet<i32> = HashSet::new();
    let n = arr.len();
    let mut res = i32::MAX;
    for &ai in arr.iter().take(n) {
        let mut cur = HashSet::new();
        for x in prev {
            cur.insert(x & ai);
        }
        cur.insert(ai);
        for &x in &cur {
            res = res.min((x - target).abs());
        }
        prev = cur;
    }
    res
}
// binary_search bit_manipulation segment_tree
#[test]
fn test2_1521() {
    assert_eq!(closest_to_target(vec![9, 12, 3, 7, 15], 5), 2);
    assert_eq!(
        closest_to_target(vec![1000000, 1000000, 1000000], 1),
        999999
    );
    assert_eq!(closest_to_target(vec![1, 2, 4, 8, 16], 0), 0);
}
