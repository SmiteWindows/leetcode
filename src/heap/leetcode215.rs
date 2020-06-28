// https://leetcode.com/problems/kth-largest-element-in-an-array/
#![allow(clippy::many_single_char_names)]
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::cmp::Ordering::{Equal, Greater, Less};
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    let n = nums.len();
    let mut l = 0;
    let mut r = n - 1;
    let k = n - k as usize;
    while l < r {
        let m = partition(&mut nums, l, r);
        match m.cmp(&k) {
            Less => {
                l = m + 1;
            }
            Greater => {
                r = m - 1;
            }
            Equal => {
                break;
            }
        }
    }
    nums[k]
}

fn partition(nums: &mut Vec<i32>, l: usize, r: usize) -> usize {
    nums.swap((l + r) / 2, r);
    let mut j = l;
    let pivot = nums[r];
    for i in l..r {
        if nums[i] <= pivot {
            nums.swap(i, j);
            j += 1;
        }
    }
    nums.swap(j, r);
    j
}
// divide_and_conquer heap
#[test]
fn test1_215() {
    assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    assert_eq!(find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
}
