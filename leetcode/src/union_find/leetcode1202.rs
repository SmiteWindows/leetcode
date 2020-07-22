// https://leetcode.com/problems/smallest-string-with-swaps/
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
    let mut res: Vec<char> = vec![];
    for i in 0..n {
        res.push(hm.entry(uf.find(i)).or_default().pop().unwrap());
    }
    res.into_iter().collect()
}

struct UnionFind {
    parents: Vec<usize>,
    n: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parents = (0..n).collect();
        Self { parents, n }
    }

    fn find(&mut self, i: usize) -> usize {
        let j = self.parents[i];
        if i == j {
            i
        } else {
            let k = self.find(j);
            self.parents[i] = k;
            k
        }
    }

    fn union(&mut self, mut i: usize, mut j: usize) {
        i = self.find(i);
        j = self.find(j);
        if i != j {
            self.parents[i] = j;
        }
    }
}
// union_find array
#[test]
fn test1_1202() {
    assert_eq!(
        smallest_string_with_swaps(String::from("dcab"), vec![vec![0, 3], vec![1, 2]]),
        String::from("bacd")
    );
    assert_eq!(
        smallest_string_with_swaps(
            String::from("dcab"),
            vec![vec![0, 3], vec![1, 2], vec![0, 2]]
        ),
        String::from("abcd")
    );
    assert_eq!(
        smallest_string_with_swaps(String::from("cba"), vec![vec![0, 1], vec![1, 2]]),
        String::from("abc")
    );
}
