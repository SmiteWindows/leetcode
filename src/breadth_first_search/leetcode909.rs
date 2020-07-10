// https://leetcode.com/problems/snakes-and-ladders/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
use std::collections::VecDeque;
pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
    let n = board.len();
    let mut moves = vec![0; n * n];
    let mut k = 0;
    for i in (0..n).rev() {
        if i % 2 != n % 2 {
            for j in 0..n {
                moves[k] = board[i][j];
                k += 1;
            }
        } else {
            for j in (0..n).rev() {
                moves[k] = board[i][j];
                k += 1;
            }
        }
    }
    let mut queue = VecDeque::new();
    let mut visited = vec![0_usize; n * n];
    visited[0] = 1;
    queue.push_back(0);
    while let Some(i) = queue.pop_front() {
        if i == n * n - 1 {
            return visited[i] as i32 - 1;
        } else {
            for (j, &mj) in moves.iter().enumerate().take(i + 6 + 1).skip(i + 1) {
                if j >= n * n {
                    break;
                }
                let k = if mj == -1 { j } else { (mj - 1) as usize };
                if visited[k] == 0 {
                    visited[k] = visited[i] + 1;
                    queue.push_back(k);
                }
            }
        }
    }
    -1
}
// breadth_first_search
#[test]
fn test1_909() {
    assert_eq!(
        snakes_and_ladders(vec![
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 35, -1, -1, 13, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 15, -1, -1, -1, -1]
        ]),
        4
    );
}
