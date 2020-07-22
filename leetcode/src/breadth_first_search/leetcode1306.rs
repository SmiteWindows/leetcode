// https://leetcode.com/problems/jump-game-iii/
// Runtime: 8 ms
// Memory Usage: 2.6 MB
use std::collections::VecDeque;
pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
    let n = arr.len();
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    visited[start as usize] = true;
    while let Some(i) = queue.pop_front() {
        if arr[i as usize] == 0 {
            return true;
        } else {
            let l = i - arr[i as usize];
            let r = i + arr[i as usize];
            if l >= 0 && !visited[l as usize] {
                visited[l as usize] = true;
                queue.push_back(l);
            }
            if r < n as i32 && !visited[r as usize] {
                visited[r as usize] = true;
                queue.push_back(r);
            }
        }
    }
    false
}
// graph breadth_first_search
#[test]
fn test2_1306() {
    assert_eq!(can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5), true);
    assert_eq!(can_reach(vec![4, 2, 3, 0, 3, 1, 2], 0), true);
    assert_eq!(can_reach(vec![3, 0, 2, 1, 2], 2), false);
}
