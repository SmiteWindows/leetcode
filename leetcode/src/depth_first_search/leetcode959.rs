// https://leetcode-cn.com/problems/regions-cut-by-slashes/
#![allow(clippy::many_single_char_names)]
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
    let a = grid
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let n = grid.len();
    let m = a[0].len();
    let mut uf = UnionFind::new(n * m * 4);
    for (i, ai) in a.iter().enumerate().take(n) {
        for j in 0..m {
            let k0 = id(0, i, j, n, m);
            let k1 = id(1, i, j, n, m);
            let k2 = id(2, i, j, n, m);
            let k3 = id(3, i, j, n, m);
            match a[i][j] {
                ' ' => {
                    uf.union(k0, k1);
                    uf.union(k1, k2);
                    uf.union(k2, k3);
                    uf.union(k3, k0);
                }
                '/' => {
                    uf.union(k0, k1);
                    uf.union(k2, k3);
                }
                '\\' => {
                    uf.union(k1, k2);
                    uf.union(k3, k0);
                }
                _ => {}
            }
            if i > 0 {
                uf.union(k1, id(3, i - 1, j, n, m));
            }
            if j > 0 {
                uf.union(k0, id(2, i, j - 1, n, m));
            }
        }
    }
    uf.n as i32
}

fn id(k: usize, i: usize, j: usize, n: usize, m: usize) -> usize {
    k * n * m + i * m + j
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
            self.parent[i] = self.find(j);
            self.parent[i]
        }
    }

    fn union(&mut self, mut i: usize, mut j: usize) {
        i = self.find(i);
        j = self.find(j);
        if i != j {
            self.parent[i] = j;
            self.n -= 1;
        }
    }
}
// union_find depth_first_search graph
#[test]
fn test3_959() {
    assert_eq!(
        regions_by_slashes(vec![String::from(" /"), String::from("/ ")]),
        2
    );
    assert_eq!(
        regions_by_slashes(vec![String::from(" /"), String::from("  ")]),
        1
    );
    assert_eq!(
        regions_by_slashes(vec![String::from("\\/"), String::from("/\\")]),
        4
    );
    assert_eq!(
        regions_by_slashes(vec![String::from("/\\"), String::from("\\/")]),
        5
    );
    assert_eq!(
        regions_by_slashes(vec![String::from("//"), String::from("/ ")]),
        3
    );
}
