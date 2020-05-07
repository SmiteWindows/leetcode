// https://leetcode.com/problems/sum-of-even-numbers-after-queries/
pub fn sum_even_after_queries(a: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_985() {
    assert_eq!(
        sum_even_after_queries(
            vec![1, 2, 3, 4],
            vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]]
        ),
        vec![8, 6, 2, 4]
    );
}
