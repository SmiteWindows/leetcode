// https://leetcode.com/problems/course-schedule-iii/
pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// greedy
#[test]
#[ignore]
fn test1_630() {
    assert_eq!(
        schedule_course(vec![
            vec![100, 200],
            vec![200, 1300],
            vec![1000, 1250],
            vec![2000, 3200]
        ]),
        3
    );
}
