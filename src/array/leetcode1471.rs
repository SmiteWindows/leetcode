// https://leetcode.com/problems/the-k-strongest-values-in-an-array/
pub fn get_strongest(arr: Vec<i32>, k: i32) -> Vec<i32> {
    todo!()
}
// array sort
#[test]
#[ignore]
fn test1_1471() {
    assert_eq!(get_strongest(vec![1, 2, 3, 4, 5], 2), vec![5, 1]);
    assert_eq!(get_strongest(vec![1, 1, 3, 5, 5], 2), vec![5, 5]);
    assert_eq!(
        get_strongest(vec![6, 7, 11, 7, 6, 8], 5),
        vec![11, 8, 6, 6, 7]
    );
    assert_eq!(get_strongest(vec![6, -3, 7, 2, 11], 3), vec![-3, 11, 2]);
    assert_eq!(get_strongest(vec![-7, 22, 17, 3], 2), vec![22, 17]);
}
