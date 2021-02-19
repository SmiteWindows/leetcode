// https://leetcode-cn.com/problems/find-the-distance-value-between-two-arrays/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
    arr1.iter()
        .filter(|&&x| arr2.iter().all(|&y| (x - y).abs() > d))
        .count() as i32
}
// array
#[test]
fn test1_1385() {
    assert_eq!(
        find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2),
        2
    );
    assert_eq!(
        find_the_distance_value(vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3),
        2
    );
    assert_eq!(
        find_the_distance_value(vec![2, 1, 100, 3], vec![-5, -2, 10, -3, 7], 6),
        1
    );
}
