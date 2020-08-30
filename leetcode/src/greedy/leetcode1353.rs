// https://leetcode-cn.com/problems/maximum-number-of-events-that-can-be-attended/
// Runtime: 56 ms
// Memory Usage: 8.7 MB
use std::{cmp::Reverse, collections::BinaryHeap};
pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
    let mut events = events;
    events.sort_by_key(|e| e[0]);
    let mut queue: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let max = events.iter().map(|e| e[1]).max().unwrap();
    let mut it = events.into_iter().peekable();
    let mut res = 0;
    for i in 1..=max {
        while let Some(&front) = queue.peek() {
            if front.0 < i {
                queue.pop();
            } else {
                break;
            }
        }
        while let Some(front) = it.peek() {
            if front[0] == i {
                queue.push(Reverse(front[1]));
                it.next();
            } else {
                break;
            }
        }
        if queue.pop().is_some() {
            res += 1;
        }
    }
    res
}
// segment_tree greedy sort
#[test]
fn test2_1353() {
    assert_eq!(max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4]]), 3);
    assert_eq!(
        max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 2]]),
        4
    );
    assert_eq!(
        max_events(vec![
            vec![1, 4],
            vec![4, 4],
            vec![2, 2],
            vec![3, 4],
            vec![1, 1]
        ]),
        4
    );
    assert_eq!(max_events(vec![vec![1, 100000]]), 1);
    assert_eq!(
        max_events(vec![
            vec![1, 1],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![1, 5],
            vec![1, 6],
            vec![1, 7]
        ]),
        7
    );
}
