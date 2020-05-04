// https://leetcode.com/problems/keys-and-rooms/
pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    todo!()
}
// graph depth_first_search
#[test]
#[ignore]
fn test2_841() {
    assert_eq!(
        can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]]),
        true
    );
    assert_eq!(
        can_visit_all_rooms(vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]]),
        false
    );
}
