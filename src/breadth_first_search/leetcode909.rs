// https://leetcode.com/problems/snakes-and-ladders/
pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// breadth_first_search
#[test]
#[ignore]
fn test1_909() {
    assert_eq!(
        snakes_and_ladders(vec![
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 35, -1, -1, 13, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 15, -1, -1, -1, -1]
        ]),
        4
    );
}
