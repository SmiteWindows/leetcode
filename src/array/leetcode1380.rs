// https://leetcode.com/problems/lucky-numbers-in-a-matrix/
pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_1380() {
    assert_eq!(
        lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]),
        vec![15]
    );
    assert_eq!(
        lucky_numbers(vec![
            vec![1, 10, 4, 2],
            vec![9, 3, 8, 7],
            vec![15, 16, 17, 12]
        ]),
        vec![12]
    );
    assert_eq!(lucky_numbers(vec![vec![7, 8], vec![1, 2]]), vec![7]);
}
