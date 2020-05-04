// https://leetcode.com/problems/combinations/
pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    todo!()
}
// backtracking
#[test]
#[ignore]
fn test1_77() {
    assert_eq!(
        combine(4, 2),
        vec![
            vec![2, 4],
            vec![3, 4],
            vec![2, 3],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4]
        ]
    );
}
