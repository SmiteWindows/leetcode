// https://leetcode-cn.com/problems/4sum-ii/
// Runtime: 36 ms
// Memory Usage: 14.9 MB
use std::collections::HashMap;
pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
    let mut hm: HashMap<i32, usize> = HashMap::new();
    for &i in &a {
        for &j in &b {
            *hm.entry(i + j).or_default() += 1;
        }
    }
    let mut res = 0;
    for &i in &c {
        for &j in &d {
            if let Some(v) = hm.get(&(-i - j)) {
                res += v;
            }
        }
    }
    res as i32
}
// binary_search hash_table
#[test]
fn test2_454() {
    assert_eq!(
        four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
        2
    );
}
