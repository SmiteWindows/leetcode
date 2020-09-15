// https://leetcode-cn.com/problems/shortest-subarray-with-sum-at-least-k/
// Runtime: 28 ms
// Memory Usage: 2.9 MB
use std::collections::VecDeque;
pub fn shortest_subarray(a: Vec<i32>, k: i32) -> i32 {
    let n = a.len();
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut prefix = vec![0; n + 1];
    queue.push_back(0);
    let mut res = usize::MAX;
    for i in 0..n {
        prefix[i + 1] = prefix[i] + a[i];
        while let Some(&j) = queue.front() {
            if prefix[i + 1] - prefix[j] >= k {
                res = res.min(i + 1 - j);
                queue.pop_front();
            } else {
                break;
            }
        }
        while let Some(&j) = queue.back() {
            if prefix[i + 1] <= prefix[j] {
                queue.pop_back();
            } else {
                break;
            }
        }
        queue.push_back(i + 1);
    }
    if res == usize::MAX {
        -1
    } else {
        res as i32
    }
}
// binary_search queue
#[test]
fn test1_862() {
    assert_eq!(shortest_subarray(vec![1], 1), 1);
    assert_eq!(shortest_subarray(vec![1, 2], 4), -1);
    assert_eq!(shortest_subarray(vec![2, -1, 2], 3), 3);
}
