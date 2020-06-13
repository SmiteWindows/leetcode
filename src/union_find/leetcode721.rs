// https://leetcode.com/problems/accounts-merge/
// Runtime: 36 ms
// Memory Usage: 5.1 MB
use std::collections::{BTreeMap, BTreeSet, HashMap};
pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let n = accounts.len();
    let mut btm: BTreeMap<&str, &str> = BTreeMap::new();
    for i in 0..n {
        let m = accounts[i].len();
        let name = &accounts[i][0];
        for j in 1..m {
            let email = &accounts[i][j];
            btm.insert(email, name);
        }
    }
    let count = btm.len();
    let mut uf = UnionFind::new(count);
    let mut owners = vec![];
    let mut emails = vec![];
    let mut ids = HashMap::new();
    for (i, (email, name)) in btm.into_iter().enumerate() {
        emails.push(email.to_string());
        owners.push(name.to_string());
        ids.insert(email.to_string(), i);
    }

    for i in 0..n {
        let m = accounts[i].len();
        for j in 2..m {
            let email_a = &accounts[i][j - 1];
            let email_b = &accounts[i][j];
            let id_a = ids[email_a];
            let id_b = ids[email_b];
            uf.union(id_a, id_b);
        }
    }
    let mut res = vec![];
    let mut hm: HashMap<usize, BTreeSet<usize>> = HashMap::new();
    for i in 0..count {
        let group_id = uf.find(i);
        hm.entry(group_id).or_default().insert(i);
    }
    for (group_id, ids) in hm.into_iter() {
        let mut v = vec![];
        v.push(owners[group_id].clone());
        for id in ids {
            v.push(emails[id].to_string());
        }
        res.push(v);
    }
    res
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
// depth_first_search union_find
#[test]
fn test1_721() {
    let mut a = accounts_merge(vec![
        vec![
            String::from("John"),
            String::from("johnsmith@mail.com"),
            String::from("john00@mail.com"),
        ],
        vec![String::from("John"), String::from("johnnybravo@mail.com")],
        vec![
            String::from("John"),
            String::from("johnsmith@mail.com"),
            String::from("john_newyork@mail.com"),
        ],
        vec![String::from("Mary"), String::from("mary@mail.com")],
    ]);
    a.sort();
    assert_eq!(
        a,
        vec![
            vec![
                String::from("John"),
                String::from("john00@mail.com"),
                String::from("john_newyork@mail.com"),
                String::from("johnsmith@mail.com")
            ],
            vec![String::from("John"), String::from("johnnybravo@mail.com")],
            vec![String::from("Mary"), String::from("mary@mail.com")]
        ]
    );
}
