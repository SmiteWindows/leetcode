// https://leetcode-cn.com/problems/network-delay-time/
#![allow(clippy::many_single_char_names)]
// Runtime: 12 ms
// Memory Usage: 2.5 MB
use std::collections::VecDeque;
pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let n = n as usize;
    let k = k as usize - 1;
    let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
    for time in times {
        let u = time[0] as usize - 1;
        let v = time[1] as usize - 1;
        let t = time[2];
        graph[u].push((v, t));
    }
    let mut visited = vec![i32::MAX; n];
    let mut queue = VecDeque::new();
    visited[k] = 0;
    queue.push_back(k);
    while let Some(u) = queue.pop_front() {
        for &(v, t) in &graph[u] {
            if t + visited[u] < visited[v] {
                visited[v] = t + visited[u];
                queue.push_back(v);
            }
        }
    }
    let max = visited.into_iter().max().unwrap();
    if max == i32::MAX {
        -1
    } else {
        max
    }
}
// graph depth_first_search breadth_first_search heap
#[test]
fn test3_743() {
    assert_eq!(
        network_delay_time(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2),
        2
    );
}
