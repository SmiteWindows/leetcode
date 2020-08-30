// https://leetcode-cn.com/problems/find-the-kth-smallest-sum-of-a-matrix-with-sorted-rows/
// Runtime: 24 ms
// Memory Usage: 4.6 MB
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};
pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut k = k;
    let n = mat.len();
    let m = mat[0].len();
    let mut queue: BinaryHeap<(Reverse<i32>, Vec<usize>)> = BinaryHeap::new();
    let mut visited: HashSet<Vec<usize>> = HashSet::new();
    let mut sum = 0;
    let indexes = vec![0; n];
    for mati in mat.iter().take(n) {
        sum += mati[0];
    }
    visited.insert(indexes.to_vec());
    queue.push((Reverse(sum), indexes));
    let mut res = 0;
    while k > 0 {
        if let Some((Reverse(sum), indexes)) = queue.pop() {
            res = sum;
            for i in 0..n {
                let mut next_sum = sum;
                let mut next_indexes = indexes.to_vec();
                let j = indexes[i];
                if j + 1 < m {
                    next_indexes[i] = j + 1;
                    if visited.insert(next_indexes.to_vec()) {
                        next_sum -= mat[i][j];
                        next_sum += mat[i][j + 1];
                        queue.push((Reverse(next_sum), next_indexes));
                    }
                }
            }
        }
        k -= 1;
    }
    res
}
// heap
#[test]
fn test1_1439() {
    assert_eq!(kth_smallest(vec![vec![1, 3, 11], vec![2, 4, 6]], 5), 7);
    assert_eq!(kth_smallest(vec![vec![1, 3, 11], vec![2, 4, 6]], 9), 17);
    assert_eq!(
        kth_smallest(vec![vec![1, 10, 10], vec![1, 4, 5], vec![2, 3, 6]], 7),
        9
    );
    assert_eq!(kth_smallest(vec![vec![1, 1, 10], vec![2, 2, 9]], 7), 12);
}
