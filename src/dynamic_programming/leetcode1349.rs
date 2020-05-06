// https://leetcode.com/problems/maximum-students-taking-exam/
pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1349() {
    assert_eq!(
        max_students(vec![
            vec!['#', '.', '#', '#', '.', '#'],
            vec!['.', '#', '#', '#', '#', '.'],
            vec!['#', '.', '#', '#', '.', '#']
        ]),
        4
    );
    assert_eq!(
        max_students(vec![
            vec!['.', '#'],
            vec!['#', '#'],
            vec!['#', '.'],
            vec!['#', '#'],
            vec!['.', '#']
        ]),
        3
    );
    assert_eq!(
        max_students(vec![
            vec!['#', '.', '.', '.', '#'],
            vec!['.', '#', '.', '#', '.'],
            vec!['.', '.', '#', '.', '.'],
            vec!['.', '#', '.', '#', '.'],
            vec!['#', '.', '.', '.', '#']
        ]),
        10
    );
}
