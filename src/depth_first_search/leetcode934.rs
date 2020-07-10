// https://leetcode.com/problems/shortest-bridge/
#[allow(clippy::many_single_char_names)]
// Runtime: 12 ms
// Memory Usage: 2.3 MB
use std::collections::VecDeque;
pub fn shortest_bridge(a: Vec<Vec<i32>>) -> i32 {
    let mut a = a;
    let n = a.len();
    let m = a[0].len();
    let mut queue = VecDeque::new();
    let mut found = false;
    for i in 0..n {
        if found {
            break;
        }
        for j in 0..m {
            if a[i][j] == 1 {
                dfs(i, j, &mut queue, &mut a, n, m);
                found = true;
                break;
            }
        }
    }
    while let Some((i, j, d)) = queue.pop_front() {
        match a[i][j] {
            0 | 2 => {
                a[i][j] = 3;
                if i > 0 && a[i - 1][j] < 2 {
                    queue.push_back((i - 1, j, d + 1));
                }
                if j > 0 && a[i][j - 1] < 2 {
                    queue.push_back((i, j - 1, d + 1));
                }
                if i + 1 < n && a[i + 1][j] < 2 {
                    queue.push_back((i + 1, j, d + 1));
                }
                if j + 1 < m && a[i][j + 1] < 2 {
                    queue.push_back((i, j + 1, d + 1));
                }
            }
            1 => {
                return d - 1;
            }
            _ => {}
        }
    }
    0
}

fn dfs(
    i: usize,
    j: usize,
    queue: &mut VecDeque<(usize, usize, i32)>,
    a: &mut Vec<Vec<i32>>,
    n: usize,
    m: usize,
) {
    if a[i][j] == 1 {
        a[i][j] = 2;
        queue.push_back((i, j, 0));
        if i > 0 {
            dfs(i - 1, j, queue, a, n, m);
        }
        if j > 0 {
            dfs(i, j - 1, queue, a, n, m);
        }
        if i + 1 < n {
            dfs(i + 1, j, queue, a, n, m);
        }
        if j + 1 < m {
            dfs(i, j + 1, queue, a, n, m);
        }
    }
}
// depth_first_search breadth_first_search
#[test]
fn test1_934() {
    assert_eq!(shortest_bridge(vec![vec![0, 1], vec![1, 0]]), 1);
    assert_eq!(
        shortest_bridge(vec![vec![0, 1, 0], vec![0, 0, 0], vec![0, 0, 1]]),
        2
    );
    assert_eq!(
        shortest_bridge(vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1]
        ]),
        1
    );
}
