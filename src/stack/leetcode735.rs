// https://leetcode.com/problems/asteroid-collision/
pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    todo!()
}
// stack
#[test]
#[ignore]
fn test1_735() {
    assert_eq!(asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
    assert_eq!(asteroid_collision(vec![8, -8]), vec![]);
    assert_eq!(asteroid_collision(vec![10, 2, -5]), vec![10]);
    assert_eq!(asteroid_collision(vec![-2, -1, 1, 2]), vec![-2, -1, 1, 2]);
}
