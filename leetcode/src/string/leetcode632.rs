// https://leetcode-cn.com/problems/smallest-range-covering-elements-from-k-lists/
// Runtime: 8 ms
// Memory Usage: 2.7 MB
use std::{cmp::Reverse, collections::BinaryHeap};
pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let n = nums.len();
    let mut queue: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
    let mut left = i32::MAX;
    let mut right = i32::MIN;
    for (i, num) in nums.iter().enumerate().take(n) {
        let x = num[0];
        left = left.min(x);
        right = right.max(x);
        queue.push((Reverse(x), i, 1));
    }
    let mut min = (right - left, (left, right));
    while let Some((Reverse(_), i, j)) = queue.pop() {
        if j == nums[i].len() {
            break;
        } else {
            let x = nums[i][j];
            right = right.max(x);
            queue.push((Reverse(x), i, j + 1));
            if let Some(&(Reverse(y), _, _)) = queue.peek() {
                left = left.max(y);
            }
            let r = (right - left, (left, right));
            if r < min {
                min = r;
            }
        }
    }
    vec![(min.1).0, (min.1).1]
}
// hash_table two_pointers string
#[test]
fn test3_632() {
    use leetcode_prelude::vec2;
    assert_eq!(
        smallest_range(vec2![[4, 10, 15, 24, 26], [0, 9, 12, 20], [5, 18, 22, 30]]),
        vec![20, 24]
    );
}
