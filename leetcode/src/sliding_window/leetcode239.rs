// https://leetcode-cn.com/problems/sliding-window-maximum/
// Runtime: 16 ms
// Memory Usage: 3.1 MB
use std::collections::VecDeque;
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    let mut queue = VecDeque::new();
    let mut res = Vec::new();
    for i in 0..n {
        let n = queue.len();
        for _ in 0..n {
            let j = queue.pop_front().unwrap();
            if i - j < k && nums[j] >= nums[i] {
                queue.push_back(j);
            }
        }
        queue.push_back(i);
        if i + 1 >= k {
            res.push(nums[*queue.front().unwrap()]);
        }
    }
    res
}
// heap sliding_window
#[test]
fn test1_239() {
    assert_eq!(
        max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
        vec![3, 3, 5, 5, 6, 7]
    );
}
