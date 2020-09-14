// https://leetcode-cn.com/problems/maximum-students-taking-exam/
pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1349() {
    use leetcode_prelude::vec2_char;
    assert_eq!(
        max_students(vec2_char![
            ['#', '.', '#', '#', '.', '#'],
            ['.', '#', '#', '#', '#', '.'],
            ['#', '.', '#', '#', '.', '#']
        ]),
        4
    );
    assert_eq!(
        max_students(vec2_char![
            ['.', '#'],
            ['#', '#'],
            ['#', '.'],
            ['#', '#'],
            ['.', '#']
        ]),
        3
    );
    assert_eq!(
        max_students(vec2_char![
            ['#', '.', '.', '.', '#'],
            ['.', '#', '.', '#', '.'],
            ['.', '.', '#', '.', '.'],
            ['.', '#', '.', '#', '.'],
            ['#', '.', '.', '.', '#']
        ]),
        10
    );
}
