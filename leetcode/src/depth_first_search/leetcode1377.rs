// https://leetcode.com/problems/frog-position-after-t-seconds/
pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
    todo!()
}
// depth_first_search
#[test]
#[ignore]
fn test1_1377() {
    assert_eq!(
        frog_position(
            7,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 7],
                vec![2, 4],
                vec![2, 6],
                vec![3, 5]
            ],
            2,
            4
        ),
        0.16666666666666666
    );
    assert_eq!(
        frog_position(
            7,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 7],
                vec![2, 4],
                vec![2, 6],
                vec![3, 5]
            ],
            1,
            7
        ),
        0.3333333333333333
    );
    assert_eq!(
        frog_position(
            7,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 7],
                vec![2, 4],
                vec![2, 6],
                vec![3, 5]
            ],
            20,
            6
        ),
        0.16666666666666666
    );
}
