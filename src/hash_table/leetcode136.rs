// https://leetcode.com/problems/single-number/
use std::collections::HashMap;
// Runtime: 0 ms
// Memory Usage: 2.4 MB
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in nums.iter() {
        *map.entry(*i).or_default() += 1;
    }
    for i in nums.iter() {
        if map[i] == 1 {
            return *i;
        }
    }
    0
}
// hash_table bit_manipulation
#[test]
fn test1_136() {
    assert_eq!(single_number(vec![2, 2, 1]), 1);
    assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
}
