// https://leetcode.com/problems/n-queens/
pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    todo!()
}
// backtracking
#[test]
#[ignore]
fn test1_51() {
    assert_eq!(
        solve_n_queens(4),
        vec![
            vec![
                String::from(".Q.."),
                String::from("...Q"),
                String::from("Q..."),
                String::from("..Q.")
            ],
            vec![
                String::from("..Q."),
                String::from("Q..."),
                String::from("...Q"),
                String::from(".Q..")
            ]
        ]
    );
}
