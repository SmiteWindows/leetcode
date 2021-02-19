// https://leetcode-cn.com/problems/the-k-weakest-rows-in-a-matrix/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::BinaryHeap;
pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut pq = BinaryHeap::new();
    for (i, m) in mat.iter().enumerate() {
        let sum = m.iter().sum::<i32>();
        pq.push((sum, i));
        if pq.len() > k as usize {
            pq.pop();
        }
    }
    let mut res = Vec::new();
    while let Some(biggest) = pq.pop() {
        res.push(biggest.1 as i32);
    }
    res.reverse();
    res
}
// binary_search array
#[test]
fn test2_1337() {
    use leetcode_prelude::vec2;
    assert_eq!(
        k_weakest_rows(
            vec2![
                [1, 1, 0, 0, 0],
                [1, 1, 1, 1, 0],
                [1, 0, 0, 0, 0],
                [1, 1, 0, 0, 0],
                [1, 1, 1, 1, 1]
            ],
            3
        ),
        vec![2, 0, 3]
    );
    assert_eq!(
        k_weakest_rows(
            vec2![[1, 0, 0, 0], [1, 1, 1, 1], [1, 0, 0, 0], [1, 0, 0, 0]],
            2
        ),
        vec![0, 2]
    );
}
