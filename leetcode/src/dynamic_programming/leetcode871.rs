// https://leetcode-cn.com/problems/minimum-number-of-refueling-stops/
// Runtime: 116 ms
// Memory Usage: 3.6 MB
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};
pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
    let mut stations = stations;
    let mut queue = BinaryHeap::new();
    stations.push(vec![target, 0]);
    let n = stations.len();
    let fuel = start_fuel - stations[0][0];
    let mut states = HashMap::new();
    if fuel >= 0 {
        states.insert((0, 0), fuel);
        queue.push((Reverse(0), 0, fuel));
    }
    while let Some((Reverse(stop), i, fuel)) = queue.pop() {
        if i == n - 1 {
            return stop as i32;
        }
        let dist = stations[i + 1][0] - stations[i][0];
        let take = fuel + stations[i][1] - dist;
        let skip = fuel - dist;
        for &(s, f) in &[(stop + 1, take), (stop, skip)] {
            if f < 0 {
                continue;
            }
            if let Some(&max_f) = states.get(&(i + 1, s)) {
                if max_f >= f {
                    continue;
                }
            }
            *states.entry((i + 1, s)).or_default() = f;
            queue.push((Reverse(s), i + 1, f));
        }
    }
    -1
}
// heap dynamic_programming
#[test]
fn test2_871() {
    use leetcode_prelude::vec2;
    assert_eq!(min_refuel_stops(1, 1, vec2![]), 0);
    assert_eq!(min_refuel_stops(100, 1, vec2![[10, 100]]), -1);
    assert_eq!(
        min_refuel_stops(100, 10, vec2![[10, 60], [20, 30], [30, 30], [60, 40]]),
        2
    );
}
