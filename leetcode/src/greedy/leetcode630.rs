// https://leetcode-cn.com/problems/course-schedule-iii/
// Runtime: 32 ms
// Memory Usage: 2.8 MB
use std::collections::BinaryHeap;
pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
    let mut courses = courses;
    courses.sort_by_key(|x| x[1]);
    let mut sum = 0;
    let mut queue = BinaryHeap::new();
    for c in courses {
        sum += c[0];
        queue.push(c[0]);
        if sum > c[1] {
            sum -= queue.pop().unwrap();
        }
    }
    queue.len() as i32
}
// greedy
#[test]
fn test1_630() {
    use leetcode_prelude::vec2;
    assert_eq!(
        schedule_course(vec2![[100, 200], [200, 1300], [1000, 1250], [2000, 3200]]),
        3
    );
}
