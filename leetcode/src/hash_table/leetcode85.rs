// https://leetcode-cn.com/problems/maximal-rectangle/
pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    todo!()
}
// stack array hash_table dynamic_programming
#[test]
#[ignore]
fn test3_85() {
    assert_eq!(
        maximal_rectangle(vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0']
        ]),
        6
    );
}
