// https://leetcode-cn.com/problems/subarray-sum-equals-k/
// Runtime: 12 ms
// Memory Usage: 2.6 MB
use std::collections::HashMap;
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut sum = 0;
    let mut hm: HashMap<i32, i32> = HashMap::new();
    let mut res = 0;
    hm.insert(0, 1);
    for x in nums {
        sum += x;
        res += hm.get(&(sum - k)).unwrap_or(&0);
        *hm.entry(sum).or_default() += 1;
    }
    res
}
// hash_table array
#[test]
fn test2_560() {
    assert_eq!(subarray_sum(vec![1, 1, 1], 2), 2);
}
