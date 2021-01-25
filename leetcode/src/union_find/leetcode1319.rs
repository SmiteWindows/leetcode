// https://leetcode-cn.com/problems/number-of-operations-to-make-network-connected/
// Runtime: 12 ms
// Memory Usage: 5.4 MB
pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let m = connections.len();
    if m + 1 < n {
        return -1;
    }
    let mut uf = UnionFind::new(n);
    for connection in connections {
        let i = connection[0] as usize;
        let j = connection[1] as usize;
        uf.union(i, j);
    }
    (uf.n - 1) as i32
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

    fn union(&mut self, i: usize, j: usize) {
        let i = self.find(i);
        let j = self.find(j);
        if i != j {
            self.parent[i] = j;
            self.n -= 1;
        }
    }
}
// union_find depth_first_search breadth_first_search
#[test]
fn test1_1319() {
    use leetcode_prelude::vec2;
    assert_eq!(make_connected(4, vec2![[0, 1], [0, 2], [1, 2]]), 1);
    assert_eq!(
        make_connected(6, vec2![[0, 1], [0, 2], [0, 3], [1, 2], [1, 3]]),
        2
    );
    assert_eq!(make_connected(6, vec2![[0, 1], [0, 2], [0, 3], [1, 2]]), -1);
    assert_eq!(make_connected(5, vec2![[0, 1], [0, 2], [3, 4], [2, 3]]), 0);
}
