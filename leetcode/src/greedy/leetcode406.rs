// https://leetcode-cn.com/problems/queue-reconstruction-by-height/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
use std::cmp::Reverse;
type People = (Reverse<i32>, i32);
pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut people = people
        .iter()
        .map(|v| (Reverse(v[0]), v[1]))
        .collect::<Vec<People>>();
    people.sort_unstable();
    let mut res = vec![];
    for p in people {
        res.insert(p.1 as usize, vec![(p.0).0, p.1]);
    }
    res
}
// greedy
#[test]
fn test1_406() {
    use leetcode_prelude::vec2;
    assert_eq!(
        reconstruct_queue(vec2![[7, 0], [4, 4], [7, 1], [5, 0], [6, 1], [5, 2]]),
        vec2![[5, 0], [7, 0], [5, 2], [6, 1], [4, 4], [7, 1]]
    );
}
