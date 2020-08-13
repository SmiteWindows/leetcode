// https://leetcode.com/problems/two-sum/
// Runtime: 0 ms
// Memory Usage: 2.4 MB
use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // 顺序扫描数组，对每一个元素，在 map 中找能组合给定值的另一半数字，
    // 如果找到了，直接返回 2 个数字的下标即可。
    // 如果找不到，就把这个数字存入 map 中，等待扫到“另一半”数字的时候，再取出来返回结果。
    let mut map = HashMap::new();

    for (i, &e) in nums.iter().enumerate() {
        if let Some(&v) = map.get(&(target - e)) {
            return vec![v, i as i32];
        }
        map.insert(e, i as i32);
    }
    vec![]
}
// array hash_table
#[test]
fn test1_1() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}
