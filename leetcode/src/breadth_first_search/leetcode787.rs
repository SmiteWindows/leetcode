// https://leetcode-cn.com/problems/cheapest-flights-within-k-stops/
// Runtime: 0 ms
// Memory Usage: 3.4 MB
use std::{cmp::Ordering, collections::BinaryHeap};
pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let n = n as usize;
    let src = src as usize;
    let dst = dst as usize;
    let k = k as usize;
    let mut prices = vec![i32::MAX; n];
    prices[src] = 0;
    let mut edges = vec![vec![]; n];
    for f in flights {
        let u = f[0] as usize;
        let v = f[1] as usize;
        let cost = f[2];
        edges[u].push(Edge { u, v, cost });
    }
    let mut pq = BinaryHeap::new();
    pq.push(State {
        position: src,
        stop: 0,
        cost: 0,
    });
    while let Some(s) = pq.pop() {
        if s.position == dst {
            return s.cost;
        }
        prices[s.position] = s.cost.min(prices[s.position]);
        if s.stop <= k {
            for e in &edges[s.position] {
                pq.push(State {
                    position: e.v,
                    cost: s.cost + e.cost,
                    stop: s.stop + 1,
                });
            }
        }
    }
    if prices[dst] == i32::MAX {
        -1
    } else {
        prices[dst]
    }
}

#[derive(Eq, Debug)]
struct State {
    position: usize,
    stop: usize,
    cost: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

#[derive(Copy, Clone)]
struct Edge {
    u: usize,
    v: usize,
    cost: i32,
}
// heap dynamic_programming breadth_first_search
#[test]
fn test2_787() {
    assert_eq!(
        find_cheapest_price(
            3,
            vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
            0,
            2,
            1
        ),
        200
    );
    assert_eq!(
        find_cheapest_price(
            3,
            vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
            0,
            2,
            0
        ),
        500
    );
}
