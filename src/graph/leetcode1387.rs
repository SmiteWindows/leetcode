// https://leetcode.com/problems/sort-integers-by-the-power-value/
pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
    todo!()
}
// graph sort
#[test]
#[ignore]
fn test1_1387() {
    assert_eq!(get_kth(12, 15, 2), 13);
    assert_eq!(get_kth(1, 1, 1), 1);
    assert_eq!(get_kth(7, 11, 4), 7);
    assert_eq!(get_kth(10, 20, 5), 13);
    assert_eq!(get_kth(1, 1000, 777), 570);
}
