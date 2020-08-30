// https://leetcode-cn.com/problems/hand-of-straights/
// Runtime: 12 ms
// Memory Usage: 2.5 MB
use std::{
    collections::{BTreeMap, VecDeque},
    iter::FromIterator,
};
pub fn is_n_straight_hand(hand: Vec<i32>, w: i32) -> bool {
    let mut btm: BTreeMap<i32, usize> = BTreeMap::new();
    for card in hand {
        *btm.entry(card).or_default() += 1;
    }
    let w = w as usize;
    let mut queue: VecDeque<(i32, usize)> = VecDeque::from_iter(btm);
    while !queue.is_empty() {
        let first = queue.pop_front().unwrap();
        let mut stack = vec![];
        for i in 1..w {
            if let Some(front) = queue.pop_front() {
                if front.0 != first.0 + i as i32 || front.1 < first.1 {
                    return false;
                } else {
                    let left = front.1 - first.1;
                    if left > 0 {
                        stack.push((front.0, left));
                    }
                }
            } else {
                return false;
            }
        }
        while let Some(last) = stack.pop() {
            queue.push_front(last);
        }
    }
    true
}
// ordered_map
#[test]
fn test1_846() {
    assert_eq!(is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3), true);
    assert_eq!(is_n_straight_hand(vec![1, 2, 3, 4, 5], 4), false);
}
