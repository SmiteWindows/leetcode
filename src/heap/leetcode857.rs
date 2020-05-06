// https://leetcode.com/problems/minimum-cost-to-hire-k-workers/
pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
    todo!()
}
// heap
#[test]
#[ignore]
fn test1_857() {
    assert_eq!(
        mincost_to_hire_workers(vec![10, 20, 5], vec![70, 50, 30], 2),
        105.00000
    );
    assert_eq!(
        mincost_to_hire_workers(vec![3, 1, 10, 10, 1], vec![4, 8, 2, 2, 7], 3),
        30.66667
    );
}
