// https://leetcode-cn.com/problems/water-and-jug-problem
// Runtime: 4 ms
// Memory Usage: 2.2 MB
use std::collections::VecDeque;
pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
    if z > x + y {
        return false;
    }
    if z == x || z == y || z == x + y {
        return true;
    }
    let mut visited = vec![false; (x + y) as usize];
    let mut queue = VecDeque::new();
    queue.push_back(0);
    while let Some(front) = queue.pop_front() {
        for diff in [x, y, -x, -y].iter() {
            let next = front + diff;
            if next == z {
                return true;
            } else if next < x + y && next > 0 && !visited[next as usize] {
                visited[next as usize] = true;
                queue.push_back(next);
            }
        }
    }
    false
}
// math
#[test]
fn test1_365() {
    assert_eq!(can_measure_water(3, 5, 4), true);
    assert_eq!(can_measure_water(2, 6, 5), false);
}
