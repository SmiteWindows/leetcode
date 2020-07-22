// https://leetcode.com/problems/rabbits-in-forest/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashMap;
pub fn num_rabbits(answers: Vec<i32>) -> i32 {
    let mut hm: HashMap<i32, i32> = HashMap::new();
    for x in answers {
        *hm.entry(x).or_default() += 1;
    }
    let mut res = 0;
    for (k, v) in hm {
        res += (v + k) / (k + 1) * (k + 1);
    }
    res
}
// math hash_table
#[test]
fn test2_781() {
    assert_eq!(num_rabbits(vec![1, 1, 2]), 5);
    assert_eq!(num_rabbits(vec![10, 10, 10]), 11);
    assert_eq!(num_rabbits(vec![]), 0);
}
