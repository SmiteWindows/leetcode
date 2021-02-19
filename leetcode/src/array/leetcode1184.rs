// https://leetcode-cn.com/problems/distance-between-bus-stops/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
    let mut start = start as usize;
    let mut destination = destination as usize;
    if destination < start {
        std::mem::swap(&mut start, &mut destination);
    }
    let n = distance.len();
    let mut left = 0;
    let mut right = 0;
    for i in 0..n {
        let j = start + i;
        if j < destination {
            left += distance[j % n];
        } else {
            right += distance[j % n];
        }
    }
    left.min(right)
}
// array
#[test]
fn test1_1184() {
    assert_eq!(distance_between_bus_stops(vec![1, 2, 3, 4], 0, 1), 1);
    assert_eq!(distance_between_bus_stops(vec![1, 2, 3, 4], 0, 2), 3);
    assert_eq!(distance_between_bus_stops(vec![1, 2, 3, 4], 0, 3), 4);
}
