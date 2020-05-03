// https://leetcode.com/problems/find-the-kth-smallest-sum-of-a-matrix-with-sorted-rows/
pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
    todo!()
}
// heap
#[test]
#[ignore]
fn test1_1439() {
    assert_eq!(kth_smallest(vec![vec![1, 3, 11], vec![2, 4, 6]], 5), 7);
    assert_eq!(kth_smallest(vec![vec![1, 3, 11], vec![2, 4, 6]], 9), 17);
    assert_eq!(
        kth_smallest(vec![vec![1, 10, 10], vec![1, 4, 5], vec![2, 3, 6]], 7),
        9
    );
    assert_eq!(kth_smallest(vec![vec![1, 1, 10], vec![2, 2, 9]], 7), 12);
}
