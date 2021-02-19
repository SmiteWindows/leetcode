// https://leetcode-cn.com/problems/n-queens/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let n = n as usize;
    let mut queens = Vec::new();
    let mut res = Vec::new();
    let mut column = 0;
    let mut diagonal1 = 0;
    let mut diagonal2 = 0;
    dfs(
        0,
        &mut queens,
        &mut column,
        &mut diagonal1,
        &mut diagonal2,
        &mut res,
        n,
    );
    res
}

fn dfs(
    i: usize,
    queens: &mut Vec<u32>,
    column: &mut u32,
    diagonal1: &mut u32,
    diagonal2: &mut u32,
    all: &mut Vec<Vec<String>>,
    n: usize,
) {
    if i == n {
        let solution = queens
            .iter()
            .map(|row| {
                (0..n)
                    .map(|i| if row & (1 << i) > 0 { 'Q' } else { '.' })
                    .collect::<String>()
            })
            .collect();
        all.push(solution);
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
                queens.push(column_bit);
                dfs(i + 1, queens, column, diagonal1, diagonal2, all, n);
                queens.pop();
                *column &= !column_bit;
                *diagonal1 &= !diagonal1_bit;
                *diagonal2 &= !diagonal2_bit;
            }
        }
    }
}
// backtracking
#[test]
fn test1_51() {
    use leetcode_prelude::vec2_string;
    assert_eq!(
        solve_n_queens(4),
        vec2_string![
            [
                ".Q..", // 解法 1
                "...Q", "Q...", "..Q."
            ],
            [
                "..Q.", // 解法 2
                "Q...", "...Q", ".Q.."
            ]
        ]
    );
}
