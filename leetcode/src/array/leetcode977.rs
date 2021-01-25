// https://leetcode-cn.com/problems/squares-of-a-sorted-array/
pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
    let mut b = a.iter().map(|&a| a.pow(2)).collect::<Vec<_>>();
    b.sort_unstable();
    b
}
// Runtime: 16 ms
// Memory Usage: 2.1 MB
// ✔
// two_pointers array
#[test]
fn test2_977() {
    assert_eq!(
        sorted_squares(vec![-4, -1, 0, 3, 10]),
        vec![0, 1, 9, 16, 100]
    );
    assert_eq!(
        sorted_squares(vec![-7, -3, 2, 3, 11]),
        vec![4, 9, 9, 49, 121]
    );
}
