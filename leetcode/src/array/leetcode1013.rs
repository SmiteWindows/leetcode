// https://leetcode-cn.com/problems/partition-array-into-three-parts-with-equal-sum/
// Runtime: 4 ms
// Memory Usage: 2.6 MB
pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
    let total = a.iter().sum::<i32>();
    let mut parts = 0;
    let mut sum = 0;
    if total % 3 != 0 {
        return false;
    }
    for x in a {
        sum += x;
        if sum == total / 3 {
            parts += 1;
            sum = 0;
        }
    }
    parts >= 3
}
// array
#[test]
fn test1_1013() {
    assert_eq!(
        can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1]),
        true
    );
    assert_eq!(
        can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1]),
        false
    );
    assert_eq!(
        can_three_parts_equal_sum(vec![3, 3, 6, 5, -2, 2, 5, 1, -9, 4]),
        true
    );
    assert_eq!(can_three_parts_equal_sum(vec![1, -1, 1, -1]), false);
    assert_eq!(
        can_three_parts_equal_sum(vec![10, -10, 10, -10, 10, -10, 10, -10]),
        true
    );
}
