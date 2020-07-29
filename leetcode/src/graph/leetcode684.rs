// https://leetcode.com/problems/redundant-connection/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = edges.len();
    let mut uf = UnionFind::new(n);
    for edge in edges {
        let u = (edge[0] - 1) as usize;
        let v = (edge[1] - 1) as usize;
        if uf.union(u, v) {
            return edge;
        }
    }
    vec![]
}

struct UnionFind {
    parent: Vec<usize>,
    n: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        Self { parent, n }
    }

    fn find(&mut self, i: usize) -> usize {
        let j = self.parent[i];
        if i == j {
            i
        } else {
            let k = self.find(j);
            self.parent[i] = k;
            k
        }
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let i = self.find(i);
        let j = self.find(j);
        if i == j {
            true
        } else {
            self.parent[i] = j;
            false
        }
    }
}
// tree graph union_find
#[test]
fn test3_684() {
    assert_eq!(
        find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
        vec![2, 3]
    );
    assert_eq!(
        find_redundant_connection(vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![1, 4],
            vec![1, 5]
        ]),
        vec![1, 4]
    );
}
