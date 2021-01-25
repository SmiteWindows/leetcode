// https://leetcode-cn.com/problems/zuma-game/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::ops::Range;
pub fn find_min_step(board: String, hand: String) -> i32 {
    let n = hand.len();
    let board: Vec<char> = board.chars().collect();
    let hand: Vec<char> = hand.chars().collect();
    let mut res = i32::MAX;
    dfs(0, 0, board, &mut res, &hand, n);
    if res == i32::MAX {
        -1
    } else {
        res
    }
}

fn dfs(start: usize, state: u32, board: Vec<char>, res: &mut i32, hand: &[char], n: usize) {
    if start == n {
        return;
    }
    for i in 0..board.len() {
        if i == 0 || board[i] != board[i - 1] {
            if let Some(next_state) = find_next_state(board[i], state, hand, n) {
                let mut new_board: Vec<char> = board.to_vec();
                new_board.insert(i, board[i]);
                while let Some(range) = dropable(&new_board) {
                    new_board.drain(range);
                    dfs(start + 1, next_state, new_board.to_vec(), res, hand, n);
                }
                if new_board.is_empty() {
                    *res = (*res).min((start + 1) as i32);
                } else {
                    dfs(start + 1, next_state, new_board, res, hand, n);
                }
            }
        }
    }
}

fn find_next_state(c: char, state: u32, hand: &[char], n: usize) -> Option<u32> {
    for (i, &hi) in hand.iter().enumerate().take(n) {
        if hi == c && state & 1 << i == 0 {
            return Some(state | 1 << i);
        }
    }
    None
}

fn dropable(board: &[char]) -> Option<Range<usize>> {
    let n = board.len();
    let mut l = 0;
    let mut r = 0;
    while r < n {
        if board[l] == board[r] {
            r += 1;
        } else if r - l >= 3 {
            return Some(l..r);
        } else {
            l = r;
        }
    }
    if r - l >= 3 {
        return Some(l..r);
    }
    None
}
// depth_first_search
#[test]
fn test1_488() {
    assert_eq!(find_min_step("WRRBBW".to_string(), "RB".to_string()), -1);
    assert_eq!(
        find_min_step("WWRRBBWW".to_string(), "WRBRW".to_string()),
        2
    );
    assert_eq!(find_min_step("G".to_string(), "GGGGG".to_string()), 2);
    assert_eq!(
        find_min_step("RBYYBBRRB".to_string(), "YRBGB".to_string()),
        3
    );
}
