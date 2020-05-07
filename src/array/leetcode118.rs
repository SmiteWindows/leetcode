// https://leetcode.com/problems/pascals-triangle/
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_118() {
    assert_eq!(
        generate(5),
        vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1]
        ]
    );
}
