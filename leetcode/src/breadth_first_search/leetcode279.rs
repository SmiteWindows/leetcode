// https://leetcode-cn.com/problems/perfect-squares/
// Runtime: 32 ms
// Memory Usage: 2.2 MB
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashSet,
};
pub fn num_squares(n: i32) -> i32 {
    let mut square_nums = Vec::new();
    let mut i = 1;
    while i * i <= n {
        square_nums.push(i * i);
        i += 1;
    }
    let mut queue = HashSet::new();
    queue.insert(n);

    let mut level = 0;
    while !queue.is_empty() {
        level += 1;
        let mut next_queue = HashSet::new();

        for &remainder in &queue {
            for &square in &square_nums {
                match remainder.cmp(&square) {
                    Less => break,
                    Equal => return level,
                    Greater => {
                        next_queue.insert(remainder - square);
                    }
                }
            }
        }
        queue = next_queue;
    }
    level
}
// math breadth_first_search dynamic_programming
#[test]
fn test3_279() {
    assert_eq!(num_squares(12), 3);
    assert_eq!(num_squares(13), 2);
}
