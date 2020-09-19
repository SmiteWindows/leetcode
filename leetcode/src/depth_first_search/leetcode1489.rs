// https://leetcode-cn.com/problems/find-critical-and-pseudo-critical-edges-in-minimum-spanning-tree/
// Runtime: 28 ms
// Memory Usage: 2 MB
pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = edges.len();
    let n = n as usize;
    let mut sorted_index: Vec<usize> = (0..m).collect();
    sorted_index.sort_unstable_by_key(|&i| edges[i][2]);
    let min_cost = mst(usize::MAX, usize::MAX, &sorted_index, &edges, n);
    let mut critical = vec![];
    let mut noncritical = vec![];
    dbg!(min_cost);
    for i in 0..m {
        if mst(i, usize::MAX, &sorted_index, &edges, n) > min_cost {
            critical.push(i as i32);
        } else if mst(usize::MAX, i, &sorted_index, &edges, n) == min_cost {
            noncritical.push(i as i32);
        }
    }
    vec![critical, noncritical]
}

fn mst(skip: usize, pick: usize, sorted_index: &[usize], edges: &[Vec<i32>], n: usize) -> i32 {
    let mut uf = UnionFind::new(n);
    let mut res = 0;
    if pick != usize::MAX && uf.union(edges[pick][0] as usize, edges[pick][1] as usize) {
        res += edges[pick][2];
    }
    for &idx in sorted_index {
        if idx != skip && uf.union(edges[idx][0] as usize, edges[idx][1] as usize) {
            res += edges[idx][2];
        }
    }
    if uf.n == 1 {
        res
    } else {
        i32::MAX
    }
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
        if i != j {
            self.parent[i] = j;
            self.n -= 1;
            true
        } else {
            false
        }
    }
}
// depth_first_search union_find
#[test]
fn test1_1489() {
    use leetcode_prelude::vec2;
    assert_eq!(
        find_critical_and_pseudo_critical_edges(
            5,
            vec2![
                [0, 1, 1],
                [1, 2, 1],
                [2, 3, 2],
                [0, 3, 2],
                [0, 4, 3],
                [3, 4, 3],
                [1, 4, 6]
            ]
        ),
        vec2![[0, 1], [2, 3, 4, 5]]
    );
    assert_eq!(
        find_critical_and_pseudo_critical_edges(
            4,
            vec2![[0, 1, 1], [1, 2, 1], [2, 3, 1], [0, 3, 1]]
        ),
        vec2![[], [0, 1, 2, 3]]
    );
}
