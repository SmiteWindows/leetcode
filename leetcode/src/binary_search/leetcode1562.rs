// https://leetcode.com/problems/find-latest-group-of-size-m/
// Runtime: 124 ms
// Memory Usage: 8.5 MB
use std::collections::HashMap;
pub fn find_latest_step(arr: Vec<i32>, m: i32) -> i32 {
    let m = m as usize;
    let n = arr.len();
    let mut values = vec![0; n];
    let mut res = -1;
    let mut uf = UnionFind::new(n);
    for i in 0..n {
        let j = (arr[i] - 1) as usize;
        values[j] = 1;
        uf.set_one(j);
        if j > 0 && values[j - 1] == 1 {
            uf.union(j - 1, j);
        }
        if j + 1 < n && values[j + 1] == 1 {
            uf.union(j, j + 1);
        }
        if *uf.group.entry(m).or_default() > 0 {
            res = (i + 1) as i32;
        }
    }
    res
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    group: HashMap<usize, usize>,
    n: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let size = vec![0; n];
        let group = HashMap::new();
        Self {
            parent,
            size,
            n,
            group,
        }
    }

    fn set_one(&mut self, i: usize) {
        self.size[i] = 1;
        *self.group.entry(1).or_default() += 1;
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

    fn union(&mut self, mut i: usize, mut j: usize) {
        i = self.find(i);
        j = self.find(j);
        let a = self.size[i];
        let b = self.size[j];
        let c = a + b;
        *self.group.entry(a).or_default() -= 1;
        *self.group.entry(b).or_default() -= 1;
        *self.group.entry(c).or_default() += 1;
        if i != j {
            self.parent[i] = j;
            self.size[j] += self.size[i];
            self.n -= 1;
        }
    }
}
// binary_search
#[test]
fn test1_1562() {
    assert_eq!(find_latest_step(vec![3, 5, 1, 2, 4], 1), 4);
    assert_eq!(find_latest_step(vec![3, 1, 5, 4, 2], 2), -1);
    assert_eq!(find_latest_step(vec![1], 1), 1);
    assert_eq!(find_latest_step(vec![2, 1], 2), 2);
}
