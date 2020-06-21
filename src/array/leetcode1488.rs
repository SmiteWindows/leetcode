// https://leetcode.com/problems/avoid-flood-in-the-city/
pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
    todo!()
}
// array hash_table
#[test]
#[ignore]
fn test1_1488() {
    assert_eq!(avoid_flood(vec![1, 2, 3, 4]), vec![-1, -1, -1, -1]);
    assert_eq!(
        avoid_flood(vec![1, 2, 0, 0, 2, 1]),
        vec![-1, -1, 2, 1, -1, -1]
    );
    assert_eq!(avoid_flood(vec![1, 2, 0, 1, 2]), vec![]);
    assert_eq!(avoid_flood(vec![69, 0, 0, 0, 69]), vec![-1, 69, 1, 1, -1]);
    assert_eq!(avoid_flood(vec![10, 20, 20]), vec![]);
}
