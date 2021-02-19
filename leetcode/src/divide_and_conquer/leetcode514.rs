// https://leetcode-cn.com/problems/freedom-trail/
// Runtime: 24 ms
// Memory Usage: 2.2 MB
use std::{cmp::Reverse, collections::BinaryHeap};
pub fn find_rotate_steps(ring: String, key: String) -> i32 {
    let n = ring.len();
    let m = key.len();
    let ring: Vec<char> = ring.chars().collect();
    let key: Vec<char> = key.chars().collect();
    let mut queue: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
    let mut dist = vec![vec![std::i32::MAX; n]; m];
    queue.push((Reverse(0), 0, 0));
    while let Some((Reverse(step), i, size)) = queue.pop() {
        if size == m {
            return step;
        }
        for (j, &rj) in ring.iter().enumerate().take(n) {
            if rj == key[size] {
                let d = step + rotate(i as i32, j as i32, n as i32) + 1;
                if d < dist[size][j] {
                    dist[size][j] = d;
                    queue.push((Reverse(d), j, size + 1));
                }
            }
        }
    }
    0
}

fn rotate(i: i32, j: i32, n: i32) -> i32 {
    let left = i - j;
    let right = j - i;
    let left = if left < 0 { left + n } else { left };
    let right = if right < 0 { right + n } else { right };
    left.min(right)
}
// divide_and_conquer dynamic_programming depth_first_search
#[test]
fn test1_514() {
    assert_eq!(
        find_rotate_steps("godding".to_string(), "gd".to_string()),
        4
    );
}
