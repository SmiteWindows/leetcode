// https://leetcode.com/problems/maximum-side-length-of-a-square-with-sum-less-than-or-equal-to-threshold/
pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
    todo!()
}
// binary_search array
#[test]
#[ignore]
fn test2_1292() {
    assert_eq!(
        max_side_length(
            vec![
                vec![1, 1, 3, 2, 4, 3, 2],
                vec![1, 1, 3, 2, 4, 3, 2],
                vec![1, 1, 3, 2, 4, 3, 2]
            ],
            4
        ),
        2
    );
    assert_eq!(
        max_side_length(
            vec![
                vec![2, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 2]
            ],
            1
        ),
        0
    );
    assert_eq!(
        max_side_length(
            vec![
                vec![1, 1, 1, 1],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0]
            ],
            6
        ),
        3
    );
    assert_eq!(
        max_side_length(
            vec![
                vec![18, 70],
                vec![61, 1],
                vec![25, 85],
                vec![14, 40],
                vec![11, 96],
                vec![97, 96],
                vec![63, 45]
            ],
            40184
        ),
        2
    );
}
