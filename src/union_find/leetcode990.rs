// https://leetcode.com/problems/satisfiability-of-equality-equations/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn equations_possible(equations: Vec<String>) -> bool {
    let mut uf = UnionFind::new(26);
    let mut pairs = vec![];
    for equation in equations {
        let s = equation.chars().collect::<Vec<_>>();
        let i = (s[0] as u8 - b'a') as usize;
        let j = (s[3] as u8 - b'a') as usize;
        if s[1] == '=' {
            uf.union(i, j);
        } else {
            pairs.push((i, j));
        }
    }
    for pair in pairs {
        let i = pair.0;
        let j = pair.1;
        if uf.find(i) == uf.find(j) {
            return false;
        }
    }
    true
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
            j
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
// union_find graph
#[test]
fn test1_990() {
    assert_eq!(
        equations_possible(vec![String::from("a==b"), String::from("b!=a")]),
        false
    );
    assert_eq!(
        equations_possible(vec![String::from("b==a"), String::from("a==b")]),
        true
    );
    assert_eq!(
        equations_possible(vec![
            String::from("a==b"),
            String::from("b==c"),
            String::from("a==c")
        ]),
        true
    );
    assert_eq!(
        equations_possible(vec![
            String::from("a==b"),
            String::from("b!=c"),
            String::from("c==a")
        ]),
        false
    );
    assert_eq!(
        equations_possible(vec![
            String::from("c==c"),
            String::from("b==d"),
            String::from("x!=z")
        ]),
        true
    );
}
