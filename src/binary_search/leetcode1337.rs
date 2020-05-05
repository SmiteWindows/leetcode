// https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/
pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    todo!()
}
// binary_search array
#[test]
#[ignore]
fn test1_1337() {
    assert_eq!(
        k_weakest_rows(
            vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 1]
            ],
            3
        ),
        vec![2, 0, 3]
    );
    assert_eq!(
        k_weakest_rows(
            vec![
                vec![1, 0, 0, 0],
                vec![1, 1, 1, 1],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0]
            ],
            2
        ),
        vec![0, 2]
    );
}
