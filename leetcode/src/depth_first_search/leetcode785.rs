// https://leetcode-cn.com/problems/is-graph-bipartite/
// Runtime: 12 ms
// Memory Usage: 2.3 MB
use std::collections::HashSet;
pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
    let n = graph.len();
    let mut g = Graph::new(n);
    for (u, gu) in graph.iter().enumerate().take(n) {
        for &v in gu {
            g.insert_edge(u, v as usize);
        }
    }
    for u in 0..n {
        if g.nodes[u] == 0 && !g.dfs(u, 1) {
            return false;
        }
    }
    true
}

struct Graph {
    edges: Vec<HashSet<usize>>,
    nodes: Vec<i32>,
    n: usize,
}

impl Graph {
    fn new(n: usize) -> Self {
        let edges = vec![HashSet::new(); n];
        let nodes = vec![0; n];
        Self { edges, nodes, n }
    }

    fn insert_edge(&mut self, u: usize, v: usize) {
        self.edges[u].insert(v);
    }

    fn dfs(&mut self, u: usize, color: i32) -> bool {
        if self.nodes[u] == 0 {
            self.nodes[u] = color;
            for v in self.edges[u].clone() {
                if !self.dfs(v, -color) {
                    return false;
                }
            }
        } else {
            return self.nodes[u] == color;
        }
        true
    }
}
// graph depth_first_search breadth_first_search
#[test]
fn test3_785() {
    use leetcode_prelude::vec2;
    assert_eq!(is_bipartite(vec2![[1, 3], [0, 2], [1, 3], [0, 2]]), true);
    assert_eq!(
        is_bipartite(vec2![[1, 2, 3], [0, 2], [0, 1, 3], [0, 2]]),
        false
    );
}
