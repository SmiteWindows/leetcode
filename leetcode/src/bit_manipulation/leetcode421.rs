// https://leetcode-cn.com/problems/maximum-xor-of-two-numbers-in-an-array/

use std::collections::HashSet;
pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut mask = 0;
    for i in (0..30).rev() {
        mask |= 1 << i;
        let mut hs = HashSet::new();
        for &x in &nums {
            hs.insert(x & mask);
        }
        let tmp = max | (1 << i);
        for &x in &hs {
            if hs.contains(&(tmp ^ x)) {
                max = tmp;
                break;
            }
        }
    }
    max
}
// TODO
// bit_manipulation trie
#[test]
fn test2_421() {
    assert_eq!(find_maximum_xor(vec![3, 10, 5, 25, 2, 8]), 28);
}
