// https://leetcode-cn.com/problems/keys-and-rooms/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::{HashSet, VecDeque};
pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    let n = rooms.len();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(0);
    while let Some(front) = queue.pop_front() {
        let room = &rooms[front];
        if visited.insert(front) {
            for &key in room {
                queue.push_back(key as usize);
            }
        }
    }
    visited.len() == n
}
// graph depth_first_search
#[test]
fn test1_841() {
    use leetcode_prelude::vec2;
    assert_eq!(can_visit_all_rooms(vec2![[1], [2], [3], []]), true);
    assert_eq!(
        can_visit_all_rooms(vec2![[1, 3], [3, 0, 1], [2], [0]]),
        false
    );
}
