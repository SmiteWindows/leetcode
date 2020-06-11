// https://leetcode.com/problems/contains-duplicate-iii/
#![allow(clippy::many_single_char_names)]
// Runtime: 4 ms
// Memory Usage: 2.1 MB
use std::collections::BTreeSet;
pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
    let mut bts = BTreeSet::new();
    let k = k as usize;
    let n = nums.len();
    if t < 0 {
        return false;
    }
    for (i, &num) in nums.iter().enumerate() {
        let l = num.checked_sub(t).unwrap_or(i32::MIN);
        let r = num.checked_add(t).unwrap_or(i32::MAX);
        if bts.range(l..=r).next().is_none() {
            bts.insert(num);
        } else {
            return true;
        }
        if i >= k {
            bts.remove(&nums[i - k]);
        }
    }
    false
}
// sort ordered_map
#[test]
fn test2_220() {
    assert_eq!(
        true,
        contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0)
    );
    assert_eq!(
        true,
        contains_nearby_almost_duplicate(vec![1, 0, 1, 1], 1, 2)
    );
    assert_eq!(
        false,
        contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3)
    );
}
