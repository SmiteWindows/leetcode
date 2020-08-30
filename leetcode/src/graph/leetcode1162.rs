// https://leetcode-cn.com/problems/as-far-from-land-as-possible/
// Runtime: 8 ms
// Memory Usage: 2.4 MB
use std::collections::VecDeque;
pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    let n = grid.len();
    let m = grid[0].len();
    let mut queue = VecDeque::new();
    for (i, gi) in grid.iter().enumerate().take(n) {
        for (j, &g) in gi.iter().enumerate().take(m) {
            if g == 1 {
                queue.push_back((i, j, 0));
            }
        }
    }
    let mut res = -1;
    let offsets = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    while let Some((i, j, d)) = queue.pop_front() {
        for offset in &offsets {
            let i = i as i32 + offset.0;
            let j = j as i32 + offset.1;
            if i >= 0 && j >= 0 && i < n as i32 && j < m as i32 {
                let i = i as usize;
                let j = j as usize;
                if grid[i][j] == 0 {
                    grid[i][j] = 1;
                    res = res.max(d + 1);
                    queue.push_back((i, j, d + 1));
                }
            }
        }
    }
    res
}
// graph breadth_first_search
#[test]
fn test1_1162() {
    assert_eq!(
        max_distance(vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]),
        2
    );
    assert_eq!(
        max_distance(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]),
        4
    );
}
