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
fn test1_1353() {
    use leetcode_prelude::vec2;
    assert_eq!(max_events(vec2![[1, 2], [2, 3], [3, 4]]), 3);
    assert_eq!(max_events(vec2![[1, 2], [2, 3], [3, 4], [1, 2]]), 4);
    assert_eq!(max_events(vec2![[1, 4], [4, 4], [2, 2], [3, 4], [1, 1]]), 4);
    assert_eq!(max_events(vec2![[1, 100000]]), 1);
    assert_eq!(
        max_events(vec2![
            [1, 1],
            [1, 2],
            [1, 3],
            [1, 4],
            [1, 5],
            [1, 6],
            [1, 7]
        ]),
        7
    );
}
