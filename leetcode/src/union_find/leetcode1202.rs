// https://leetcode-cn.com/problems/smallest-string-with-swaps/
// Runtime: 28 ms
// Memory Usage: 9 MB
use std::{cmp::Reverse, collections::HashMap};
pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
    let n = s.len();
    let mut uf = UnionFind::new(n);
    let mut hm: HashMap<usize, Vec<char>> = HashMap::new();
    for pair in pairs {
        uf.union(pair[0] as usize, pair[1] as usize);
    }
    for (i, c) in s.char_indices() {
        hm.entry(uf.find(i)).or_default().push(c);
    }
    for v in hm.values_mut() {
        v.sort_unstable_by_key(|&x| Reverse(x));
    }
    let mut res: Vec<char> = Vec::new();
    for i in 0..n {
        res.push(hm.entry(uf.find(i)).or_default().pop().unwrap());
    }
    res.into_iter().collect()
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

    fn union(&mut self, mut i: usize, mut j: usize) {
        i = self.find(i);
        j = self.find(j);
        if i != j {
            self.parent[i] = j;
        }
    }
}
// union_find array
#[test]
fn test1_1202() {
    use leetcode_prelude::vec2;
    assert_eq!(
        smallest_string_with_swaps("dcab".to_string(), vec2![[0, 3], [1, 2]]),
        "bacd".to_string()
    );
    assert_eq!(
        smallest_string_with_swaps("dcab".to_string(), vec2![[0, 3], [1, 2], [0, 2]]),
        "abcd".to_string()
    );
    assert_eq!(
        smallest_string_with_swaps("cba".to_string(), vec2![[0, 1], [1, 2]]),
        "abc".to_string()
    );
}
