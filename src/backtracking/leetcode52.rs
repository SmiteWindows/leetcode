// https://leetcode.com/problems/n-queens-ii/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn total_n_queens(n: i32) -> i32 {
    let n = n as usize;
    let mut column: u32 = 0;
    let mut diagonal1: u32 = 0;
    let mut diagonal2: u32 = 0;
    let mut res = 0;
    dfs(0, &mut column, &mut diagonal1, &mut diagonal2, &mut res, n);
    res
}

fn dfs(
    i: usize,
    column: &mut u32,
    diagonal1: &mut u32,
    diagonal2: &mut u32,
    count: &mut i32,
    n: usize,
) {
    if i == n {
        *count += 1;
    } else {
        for j in 0..n {
            let column_bit = 1 << j;
            let diagonal1_bit = 1 << (i + j);
            let diagonal2_bit = 1 << (n + i - j);
            if column_bit & *column == 0
                && diagonal1_bit & *diagonal1 == 0
                && diagonal2_bit & *diagonal2 == 0
            {
                *column |= column_bit;
                *diagonal1 |= diagonal1_bit;
                *diagonal2 |= diagonal2_bit;
                dfs(i + 1, column, diagonal1, diagonal2, count, n);
                *column &= !column_bit;
                *diagonal1 &= !diagonal1_bit;
                *diagonal2 &= !diagonal2_bit;
            }
        }
    }
}
// backtracking
#[test]
fn test1_52() {
    assert_eq!(total_n_queens(4), 2);
}
