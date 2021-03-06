// https://leetcode-cn.com/problems/bus-routes/
// Runtime: 56 ms
// Memory Usage: 17.7 MB
use std::collections::{HashMap, HashSet, VecDeque};
pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, s: i32, t: i32) -> i32 {
    if s == t {
        return 0;
    }
    let mut buses: HashMap<i32, HashSet<usize>> = HashMap::new();
    for (i, route) in routes.iter().enumerate() {
        for &j in route {
            buses.entry(j).or_default().insert(i);
        }
    }
    let mut queue: VecDeque<i32> = VecDeque::new();
    let mut visited: HashSet<usize> = HashSet::new();
    queue.push_back(s);
    let mut res = 0;
    while !queue.is_empty() {
        let size = queue.len();
        for _ in 0..size {
            let u = queue.pop_front().unwrap();
            if u == t {
                return res;
            }
            let mut stops = HashSet::new();
            for &bus in &buses[&u] {
                if visited.insert(bus) {
                    for &i in &routes[bus] {
                        stops.insert(i);
                    }
                }
            }
            for stop in stops {
                queue.push_back(stop);
            }
        }
        res += 1;
    }
    -1
}
// breadth_first_search
#[test]
fn test1_815() {
    use leetcode_prelude::vec2;
    assert_eq!(
        num_buses_to_destination(vec2![[1, 2, 7], [3, 6, 7]], 1, 6),
        2
    );
}
