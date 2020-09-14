// https://leetcode-cn.com/problems/minesweeper/
// Runtime: 20 ms
// Memory Usage: 2.2 MB
use std::collections::VecDeque;
pub fn update_board(board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
    let mut board = board;
    let n = board.len();
    let m = board[0].len();
    let i = click[0] as usize;
    let j = click[1] as usize;
    if board[i][j] == 'M' {
        board[i][j] = 'X';
        return board;
    }
    let mut visited = vec![vec![false; m]; n];
    let mut queue = VecDeque::new();
    queue.push_back(Point { i, j });
    while let Some(p) = queue.pop_front() {
        visited[p.i][p.j] = true;
        if board[p.i][p.j] == 'E' {
            let mut sum = 0;
            for q in p.adj(n, m) {
                if board[q.i][q.j] == 'M' {
                    sum += 1;
                }
            }
            if sum == 0 {
                board[p.i][p.j] = 'B';
                for q in p.adj(n, m) {
                    if !visited[q.i][q.j] {
                        queue.push_back(q);
                    }
                }
            } else {
                board[p.i][p.j] = (b'0' + sum) as char;
            }
        }
    }
    board
}

struct Point {
    i: usize,
    j: usize,
}

macro_rules! point {
    ($i:expr,$j:expr) => {
        Point { i: $i, j: $j }
    };
}

impl Point {
    fn adj(&self, n: usize, m: usize) -> Vec<Point> {
        let mut res = vec![];
        for i in -1..=1 {
            for j in -1..=1 {
                let r = self.i as i32 + i;
                let c = self.j as i32 + j;
                if r < 0 || r > (n - 1) as i32 || c < 0 || c > (m - 1) as i32 {
                    continue;
                }
                res.push(point!(r as usize, c as usize))
            }
        }
        res
    }
}
// depth_first_search breadth_first_search
#[test]
fn test1_529() {
    use leetcode_prelude::vec2_char;
    assert_eq!(
        update_board(
            vec2_char![
                ['E', 'E', 'E', 'E', 'E'],
                ['E', 'E', 'M', 'E', 'E'],
                ['E', 'E', 'E', 'E', 'E'],
                ['E', 'E', 'E', 'E', 'E']
            ],
            vec![3, 0]
        ),
        vec2_char![
            ['B', '1', 'E', '1', 'B'],
            ['B', '1', 'M', '1', 'B'],
            ['B', '1', '1', '1', 'B'],
            ['B', 'B', 'B', 'B', 'B']
        ]
    );
    assert_eq!(
        update_board(
            vec2_char![
                ['B', '1', 'E', '1', 'B'],
                ['B', '1', 'M', '1', 'B'],
                ['B', '1', '1', '1', 'B'],
                ['B', 'B', 'B', 'B', 'B']
            ],
            vec![1, 2]
        ),
        vec2_char![
            ['B', '1', 'E', '1', 'B'],
            ['B', '1', 'X', '1', 'B'],
            ['B', '1', '1', '1', 'B'],
            ['B', 'B', 'B', 'B', 'B']
        ]
    );
}
