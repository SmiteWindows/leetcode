// https://leetcode.com/problems/path-with-maximum-probability/
pub fn max_probability(
    n: i32,
    edges: Vec<Vec<i32>>,
    succ_prob: Vec<f64>,
    start: i32,
    end: i32,
) -> f64 {
    todo!()
}
// graph
#[test]
#[ignore]
fn test1_1514() {
    assert_eq!(
        max_probability(
            3,
            vec![vec![0, 1], vec![1, 2], vec![0, 2]],
            vec![0.5, 0.5, 0.2],
            0,
            2
        ),
        0.25000
    );
    assert_eq!(
        max_probability(
            3,
            vec![vec![0, 1], vec![1, 2], vec![0, 2]],
            vec![0.5, 0.5, 0.3],
            0,
            2
        ),
        0.30000
    );
    assert_eq!(
        max_probability(3, vec![vec![0, 1]], vec![0.5], 0, 2),
        0.00000
    );
}
