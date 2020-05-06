// https://leetcode.com/problems/maximal-square/
pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_221() {
    assert_eq!(
        maximal_square(vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0']
        ]),
        4
    );
}
