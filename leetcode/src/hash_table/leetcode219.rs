// https://leetcode.com/problems/contains-duplicate-ii/
// Runtime: 4 ms
// Memory Usage: 2.8 MB
use std::collections::HashSet;
pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut hs = HashSet::new();
    let k = k as usize;
    for (i, &x) in nums.iter().enumerate() {
        if hs.contains(&x) {
            return true;
        } else {
            hs.insert(x);
            if hs.len() > k {
                hs.remove(&nums[i - k]);
            }
        }
    }
    false
}
// hash_table array
#[test]
fn test1_219() {
    assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1], 3), true);
    assert_eq!(contains_nearby_duplicate(vec![1, 0, 1, 1], 1), true);
    assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2), false);
}
