// https://leetcode.com/problems/interval-list-intersections/
pub fn interval_intersection(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    todo!()
}
// two_pointers
#[test]
#[ignore]
fn test1_986() {
    assert_eq!(
        interval_intersection(
            vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]],
            vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]]
        ),
        vec![
            vec![1, 2],
            vec![5, 5],
            vec![8, 10],
            vec![15, 23],
            vec![24, 24],
            vec![25, 25]
        ]
    );
}
