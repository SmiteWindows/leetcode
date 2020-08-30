// https://leetcode-cn.com/problems/sort-integers-by-the-power-value/
// Runtime: 8 ms
// Memory Usage: 2.1 MB
use std::collections::BinaryHeap;
pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
    let k = k as usize;
    let mut pq = BinaryHeap::new();
    for i in lo..=hi {
        pq.push((power(i), i));
        if pq.len() > k {
            pq.pop();
        }
    }
    pq.pop().unwrap().1
}

fn power(mut num: i32) -> i32 {
    let mut res = 0;
    while num != 1 {
        res += 1;
        if num % 2 == 0 {
            num /= 2;
        } else {
            num = 3 * num + 1;
        }
    }
    res
}
// graph sort
#[test]
fn test1_1387() {
    assert_eq!(get_kth(12, 15, 2), 13);
    assert_eq!(get_kth(1, 1, 1), 1);
    assert_eq!(get_kth(7, 11, 4), 7);
    assert_eq!(get_kth(10, 20, 5), 13);
    assert_eq!(get_kth(1, 1000, 777), 570);
}
