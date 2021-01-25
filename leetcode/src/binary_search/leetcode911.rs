// https://leetcode-cn.com/problems/online-election/
// Runtime: 72 ms
// Memory Usage: 5.9 MB
use std::collections::HashMap;
struct TopVotedCandidate {
    times: Vec<i32>,
    leaders: Vec<i32>,
}

impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let n = persons.len();
        let mut hm: HashMap<i32, usize> = HashMap::new();
        let mut leaders = vec![];
        let mut leader = (0, 0);
        for &p in &persons {
            let count = hm.entry(p).or_default();
            *count += 1;
            if *count >= leader.0 {
                leader = (*count, p);
            }
            leaders.push(leader.1);
        }
        Self { times, leaders }
    }

    fn q(&self, t: i32) -> i32 {
        let i = match self.times.binary_search(&t) {
            Ok(i) => i,
            Err(i) => i - 1,
        };
        self.leaders[i]
    }
}
/**
 * Your TopVotedCandidate object will be instantiated and called as such:
 * let obj = TopVotedCandidate::new(persons, times);
 * let ret_1: i32 = obj.q(t);
 */
// binary_search
#[test]
fn test1_911() {
    let persons = vec![0, 1, 1, 0, 0, 1, 0];
    let times = vec![0, 5, 10, 15, 20, 25, 30];
    let tvc = TopVotedCandidate::new(persons, times);
    assert_eq!(tvc.q(3), 0);
    assert_eq!(tvc.q(12), 1);
    assert_eq!(tvc.q(25), 1);
    assert_eq!(tvc.q(15), 0);
    assert_eq!(tvc.q(24), 0);
    assert_eq!(tvc.q(8), 1);
}
