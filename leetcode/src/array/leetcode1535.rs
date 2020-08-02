// https://leetcode.com/problems/find-the-winner-of-an-array-game/
// Runtime: 12 ms
// Memory Usage: 3.4 MB
use std::collections::VecDeque;
pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
    let mut queue = VecDeque::new();
    let k = k as usize;
    let k = k.min(arr.len());
    for x in arr {
        queue.push_back(x);
    }
    loop {
        let first = queue.pop_front().unwrap();
        let second = queue.pop_front().unwrap();
        if second > first {
            queue.push_front(first);
            queue.push_front(second);
        } else {
            queue.push_front(second);
            queue.push_front(first);
        }
        let mut round = 0;
        let winner = queue.pop_front().unwrap();
        while let Some(first) = queue.pop_front() {
            if first < winner {
                queue.push_back(first);
                round += 1;
                if round == k {
                    return winner;
                }
            } else {
                queue.push_front(first);
                queue.push_front(winner);
                break;
            }
        }
    }
}
// array
#[test]
fn test1_1535() {
    assert_eq!(get_winner(vec![2, 1, 3, 5, 4, 6, 7], 2), 5);
    assert_eq!(get_winner(vec![3, 2, 1], 10), 3);
    assert_eq!(get_winner(vec![1, 9, 8, 2, 3, 7, 6, 4, 5], 7), 9);
    assert_eq!(
        get_winner(vec![1, 11, 22, 33, 44, 55, 66, 77, 88, 99], 1000000000),
        99
    );
}
