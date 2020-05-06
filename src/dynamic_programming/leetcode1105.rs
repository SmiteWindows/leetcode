// https://leetcode.com/problems/filling-bookcase-shelves/
pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1105() {
    assert_eq!(
        min_height_shelves(
            vec![
                vec![1, 1],
                vec![2, 3],
                vec![2, 3],
                vec![1, 1],
                vec![1, 1],
                vec![1, 1],
                vec![1, 2]
            ],
            4
        ),
        6
    );
}
