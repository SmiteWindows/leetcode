// https://leetcode-cn.com/problems/number-of-ways-to-wear-different-hats-to-each-other/
pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// bit_manipulation dynamic_programming
#[test]
#[ignore]
fn test2_1434() {
    assert_eq!(number_ways(vec![vec![3, 4], vec![4, 5], vec![5]]), 1);
    assert_eq!(number_ways(vec![vec![3, 5, 1], vec![3, 5]]), 4);
    assert_eq!(
        number_ways(vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4]
        ]),
        24
    );
    assert_eq!(
        number_ways(vec![
            vec![1, 2, 3],
            vec![2, 3, 5, 6],
            vec![1, 3, 7, 9],
            vec![1, 8, 9],
            vec![2, 5, 7]
        ]),
        111
    );
}
