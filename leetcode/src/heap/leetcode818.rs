// https://leetcode.com/problems/race-car/
// Runtime: 132 ms
// Memory Usage: 8.5 MB
use std::collections::{HashMap, VecDeque};
pub fn racecar(target: i32) -> i32 {
    let mut queue = VecDeque::new();
    let mut states = HashMap::new();
    queue.push_back((0, 0, 1));
    states.insert((0, 1), 0);
    while let Some((length, position, speed)) = queue.pop_front() {
        if position == target {
            return length as i32;
        }

        for (p, s) in next(position, speed) {
            if p > target + target / 3 || p < -target / 3 {
                continue;
            }
            if states.contains_key(&(p, s)) && states[&(p, s)] <= length + 1 {
                continue;
            }
            *states.entry((p, s)).or_default() = length + 1;
            queue.push_back((length + 1, p, s));
        }
    }
    0
}

fn next(position: i32, speed: i32) -> Vec<(i32, i32)> {
    vec![
        (position + speed, speed * 2),
        (position, if speed > 0 { -1 } else { 1 }),
    ]
}
// heap dynamic_programming
#[test]
fn test1_818() {
    assert_eq!(racecar(3), 2);
    assert_eq!(racecar(6), 5);
}
