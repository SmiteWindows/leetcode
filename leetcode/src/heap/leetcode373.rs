// https://leetcode-cn.com/problems/find-k-pairs-with-smallest-sums/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};
pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, mut k: i32) -> Vec<Vec<i32>> {
    let n1 = nums1.len();
    let n2 = nums2.len();
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    if 0 < n1 && 0 < n2 && visited.insert((0, 0)) {
        queue.push((Reverse(nums1[0] + nums2[0]), 0, 0));
    } else {
        return Vec::new();
    }
    let mut res = Vec::new();
    while k > 0 {
        if let Some((_, i, j)) = queue.pop() {
            res.push(vec![nums1[i], nums2[j]]);
            if i + 1 < n1 && visited.insert((i + 1, j)) {
                queue.push((Reverse(nums1[i + 1] + nums2[j]), i + 1, j));
            }
            if j + 1 < n2 && visited.insert((i, j + 1)) {
                queue.push((Reverse(nums1[i] + nums2[j + 1]), i, j + 1));
            }
            k -= 1;
        } else {
            break;
        }
    }
    res
}
// heap
#[test]
fn test1_373() {
    use leetcode_prelude::vec2;
    assert_eq!(
        k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
        vec2![[1, 2], [1, 4], [1, 6]]
    );
    assert_eq!(
        k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2),
        vec2![[1, 1], [1, 1]]
    );
    assert_eq!(
        k_smallest_pairs(vec![1, 2], vec![3], 3),
        vec2![[1, 3], [2, 3]]
    );
}
