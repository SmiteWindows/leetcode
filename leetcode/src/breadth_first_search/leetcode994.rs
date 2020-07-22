// https://leetcode.com/problems/rotting-oranges/
#![allow(clippy::many_single_char_names)]
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::collections::VecDeque;
pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    let n = grid.len();
    let m = grid[0].len();
    let mut queue = VecDeque::new();
    for (i, gi) in grid.iter().enumerate().take(n) {
        for (j, &gij) in gi.iter().enumerate().take(m) {
            if gij == 2 {
                queue.push_back(Orange { r: i, c: j, t: 0 });
            }
        }
    }
    let mut res = 0;
    while let Some(o) = queue.pop_front() {
        let r = o.r;
        let c = o.c;
        let t = o.t;
        if r > 0 && grid[r - 1][c] == 1 {
            grid[r - 1][c] = 2;
            res = res.max(t + 1);
            queue.push_back(Orange {
                r: r - 1,
                c,
                t: t + 1,
            });
        }
        if r < n - 1 && grid[r + 1][c] == 1 {
            grid[r + 1][c] = 2;
            res = res.max(t + 1);
            queue.push_back(Orange {
                r: r + 1,
                c,
                t: t + 1,
            });
        }
        if c > 0 && grid[r][c - 1] == 1 {
            grid[r][c - 1] = 2;
            res = res.max(t + 1);
            queue.push_back(Orange {
                r,
                c: c - 1,
                t: t + 1,
            });
        }
        if c < m - 1 && grid[r][c + 1] == 1 {
            grid[r][c + 1] = 2;
            res = res.max(t + 1);
            queue.push_back(Orange {
                r,
                c: c + 1,
                t: t + 1,
            });
        }
    }
    for gi in grid.iter().take(n) {
        for &gij in gi.iter().take(m) {
            if gij == 1 {
                return -1;
            }
        }
    }
    res
}

struct Orange {
    r: usize,
    c: usize,
    t: i32,
}
// breadth_first_search
#[test]
fn test1_994() {
    assert_eq!(
        oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]),
        4
    );
    assert_eq!(
        oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]),
        -1
    );
    assert_eq!(oranges_rotting(vec![vec![0, 2]]), 0);
}
