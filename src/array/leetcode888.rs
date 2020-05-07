// https://leetcode.com/problems/fair-candy-swap/
pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_888() {
    assert_eq!(fair_candy_swap(vec![1, 1], vec![2, 2]), vec![1, 2]);
    assert_eq!(fair_candy_swap(vec![1, 2], vec![2, 3]), vec![1, 2]);
    assert_eq!(fair_candy_swap(vec![2], vec![1, 3]), vec![2, 3]);
    assert_eq!(fair_candy_swap(vec![1, 2, 5], vec![2, 4]), vec![5, 4]);
}
