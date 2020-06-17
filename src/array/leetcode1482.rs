// https://leetcode.com/problems/minimum-number-of-days-to-make-m-bouquets/
pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
    todo!()
}
// array binary_search
#[test]
#[ignore]
fn test1_1482() {
    assert_eq!(min_days(vec![1, 10, 3, 10, 2], 3, 1), 3);
    assert_eq!(min_days(vec![1, 10, 3, 10, 2], 3, 2), -1);
    assert_eq!(min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3), 12);
    assert_eq!(min_days(vec![1000000000, 1000000000], 1, 1), 1000000000);
    assert_eq!(min_days(vec![1, 10, 2, 9, 3, 8, 4, 7, 5, 6], 4, 2), 9);
}
