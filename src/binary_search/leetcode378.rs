// https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/
pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    todo!()
}
// binary_search heap
#[test]
#[ignore]
fn test1_378() {
    assert_eq!(
        kth_smallest(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8),
        13
    );
}
