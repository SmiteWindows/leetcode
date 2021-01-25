// https://leetcode-cn.com/problems/minimum-swaps-to-arrange-a-binary-grid/
// Runtime: 8 ms
// Memory Usage: 2.3 MB
pub fn min_swaps(mut grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut res = 0;
    'outer: for i in 0..n {
        for j in i..n {
            if grid[j][i + 1..].iter().all(|&x| x == 0) {
                let mut k = j;
                while k > i {
                    grid.swap(k, k - 1);
                    res += 1;
                    k -= 1;
                }
                continue 'outer;
            }
        }
        return -1;
    }
    res as i32
}
// greedy
#[test]
fn test1_1536() {
    use leetcode_prelude::vec2;
    assert_eq!(min_swaps(vec2![[0, 0, 1], [1, 1, 0], [1, 0, 0]]), 3);
    assert_eq!(
        min_swaps(vec2![
            [0, 1, 1, 0],
            [0, 1, 1, 0],
            [0, 1, 1, 0],
            [0, 1, 1, 0]
        ]),
        -1
    );
    assert_eq!(min_swaps(vec2![[1, 0, 0], [1, 1, 0], [1, 1, 1]]), 0);
}
