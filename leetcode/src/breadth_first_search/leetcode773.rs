// https://leetcode-cn.com/problems/sliding-puzzle/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
use std::collections::{HashSet, VecDeque};
pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
    let mut visited = HashSet::new();
    let next = vec![
        vec![1, 3],
        vec![0, 4, 2],
        vec![1, 5],
        vec![0, 4],
        vec![3, 1, 5],
        vec![2, 4],
    ];
    let solved = vec![1, 2, 3, 4, 5, 0];
    let mut queue = VecDeque::new();
    let mut line = Vec::new();
    board
        .into_iter()
        .for_each(|v| v.into_iter().for_each(|x| line.push(x)));
    let zero = line.iter().position(|&x| x == 0).unwrap();
    visited.insert(line.to_vec());
    queue.push_back((line, zero, 0));
    while let Some((line, zero, count)) = queue.pop_front() {
        if line == solved {
            return count;
        }
        for &index in &next[zero] {
            let mut copy = line.to_vec();
            copy.swap(index, zero);
            if visited.insert(copy.to_vec()) {
                queue.push_back((copy, index, count + 1));
            }
        }
    }
    -1
}
// breadth_first_search
#[test]
fn test1_773() {
    use leetcode_prelude::vec2;
    assert_eq!(sliding_puzzle(vec2![[1, 2, 3], [4, 0, 5]]), 1);
    assert_eq!(sliding_puzzle(vec2![[1, 2, 3], [5, 4, 0]]), -1);
    assert_eq!(sliding_puzzle(vec2![[4, 1, 2], [5, 0, 3]]), 5);
    assert_eq!(sliding_puzzle(vec2![[3, 2, 4], [1, 5, 0]]), 14);
}
