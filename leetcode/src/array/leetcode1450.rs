// https://leetcode.com/problems/number-of-students-doing-homework-at-a-given-time/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
    let n = start_time.len();
    let mut res = 0;
    for i in 0..n {
        let start = start_time[i];
        let end = end_time[i];
        if start <= query_time && end >= query_time {
            res += 1;
        }
    }
    res
}
// array
#[test]
fn test1_1450() {
    assert_eq!(busy_student(vec![1, 2, 3], vec![3, 2, 7], 4), 1);
    assert_eq!(busy_student(vec![4], vec![4], 4), 1);
    assert_eq!(busy_student(vec![4], vec![4], 5), 0);
    assert_eq!(busy_student(vec![1, 1, 1, 1], vec![1, 3, 2, 4], 7), 0);
    assert_eq!(
        busy_student(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
            vec![10, 10, 10, 10, 10, 10, 10, 10, 10],
            5
        ),
        5
    );
}
