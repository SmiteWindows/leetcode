// https://leetcode-cn.com/problems/reveal-cards-in-increasing-order/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::collections::VecDeque;
pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
    let mut deck = deck;
    deck.sort_unstable();
    let mut queue = VecDeque::new();
    let n = deck.len();
    for i in (0..n).rev() {
        let last = deck[i];
        if let Some(bottom) = queue.pop_back() {
            queue.push_front(bottom);
        }
        queue.push_front(last);
    }
    queue.into_iter().collect()
}
// array
#[test]
fn test1_950() {
    assert_eq!(
        deck_revealed_increasing(vec![17, 13, 11, 2, 3, 5, 7]),
        vec![2, 13, 3, 11, 5, 17, 7]
    );
}
