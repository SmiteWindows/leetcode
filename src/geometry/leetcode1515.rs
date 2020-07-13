// https://leetcode.com/problems/best-position-for-a-service-centre/
pub fn get_min_dist_sum(positions: Vec<Vec<i32>>) -> f64 {
    todo!()
}
// geometry
#[test]
#[ignore]
fn test1_1515() {
    assert_eq!(
        get_min_dist_sum(vec![vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 1]]),
        4.00000
    );
    assert_eq!(get_min_dist_sum(vec![vec![1, 1], vec![3, 3]]), 2.82843);
    assert_eq!(get_min_dist_sum(vec![vec![1, 1]]), 0.00000);
    assert_eq!(
        get_min_dist_sum(vec![vec![1, 1], vec![0, 0], vec![2, 0]]),
        2.73205
    );
    assert_eq!(
        get_min_dist_sum(vec![
            vec![0, 1],
            vec![3, 2],
            vec![4, 5],
            vec![7, 6],
            vec![8, 9],
            vec![11, 1],
            vec![2, 12]
        ]),
        32.94036
    );
}
