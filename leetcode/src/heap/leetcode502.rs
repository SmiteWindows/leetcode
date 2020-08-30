// https://leetcode-cn.com/problems/ipo/
// Runtime: 4 ms
// Memory Usage: 3.6 MB
use std::collections::BinaryHeap;
pub fn find_maximized_capital(mut k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    let mut sorted_capital: Vec<(i32, usize)> = vec![];
    for (i, &ci) in capital.iter().enumerate() {
        sorted_capital.push((ci, i));
    }
    sorted_capital.sort_unstable();
    sorted_capital.reverse();
    let mut res = w;
    let mut queue: BinaryHeap<i32> = BinaryHeap::new();
    loop {
        while let Some(&(c, i)) = sorted_capital.last() {
            if c <= res {
                sorted_capital.pop();
                queue.push(profits[i]);
            } else {
                break;
            }
        }
        if let Some(max) = queue.pop() {
            res += max;
            k -= 1;
        } else {
            break;
        }
        if k == 0 {
            break;
        }
    }
    res
}
// heap greedy
#[test]
fn test1_502() {
    assert_eq!(
        find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]),
        4
    );
}
