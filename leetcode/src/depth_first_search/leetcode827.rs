// https://leetcode-cn.com/problems/making-a-large-island/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
use std::collections::HashMap;
pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut group = vec![vec![0; n]; n];
    let mut gid = 0;
    let mut group_size = vec![0];
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 1 && group[i][j] == 0 {
                gid += 1;
                group_size.push(0);
                dfs(i, j, gid, &mut group, &mut group_size, &grid, n);
            }
        }
    }
    let mut res = *group_size.iter().max().unwrap_or(&0);
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 0 {
                let mut groups: HashMap<usize, usize> = HashMap::new();
                if i > 0 {
                    let gid = group[i - 1][j];
                    let size = group_size[gid];
                    groups.entry(gid).or_insert(size);
                }
                if j > 0 {
                    let gid = group[i][j - 1];
                    let size = group_size[gid];
                    groups.entry(gid).or_insert(size);
                }
                if i + 1 < n {
                    let gid = group[i + 1][j];
                    let size = group_size[gid];
                    groups.entry(gid).or_insert(size);
                }
                if j + 1 < n {
                    let gid = group[i][j + 1];
                    let size = group_size[gid];
                    groups.entry(gid).or_insert(size);
                }
                res = res.max(groups.values().sum::<usize>() + 1);
            }
        }
    }
    res as i32
}

fn dfs(
    i: usize,
    j: usize,
    gid: usize,
    group: &mut Vec<Vec<usize>>,
    group_size: &mut Vec<usize>,
    grid: &[Vec<i32>],
    n: usize,
) {
    group[i][j] = gid;
    group_size[gid] += 1;
    if i > 0 && grid[i - 1][j] == 1 && group[i - 1][j] == 0 {
        dfs(i - 1, j, gid, group, group_size, grid, n);
    }
    if j > 0 && grid[i][j - 1] == 1 && group[i][j - 1] == 0 {
        dfs(i, j - 1, gid, group, group_size, grid, n);
    }
    if i + 1 < n && grid[i + 1][j] == 1 && group[i + 1][j] == 0 {
        dfs(i + 1, j, gid, group, group_size, grid, n);
    }
    if j + 1 < n && grid[i][j + 1] == 1 && group[i][j + 1] == 0 {
        dfs(i, j + 1, gid, group, group_size, grid, n);
    }
}
// depth_first_search
#[test]
fn test1_827() {
    use leetcode_prelude::vec2;
    assert_eq!(largest_island(vec2![[1, 0], [0, 1]]), 3);
    assert_eq!(largest_island(vec2![[1, 1], [1, 0]]), 4);
    assert_eq!(largest_island(vec2![[1, 1], [1, 1]]), 4);
}
