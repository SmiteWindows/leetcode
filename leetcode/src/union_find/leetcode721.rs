// https://leetcode.com/problems/accounts-merge/
// Runtime: 36 ms
// Memory Usage: 5.1 MB
use std::collections::{BTreeMap, BTreeSet, HashMap};
pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut btm: BTreeMap<&str, &str> = BTreeMap::new();
    for account in accounts.iter() {
        let name = &account[0];
        for a in account.iter().skip(1) {
            let email = &a;
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

    for account in accounts.iter() {
        let m = account.len();
        for j in 2..m {
            let email_a = &account[j - 1];
            let email_b = &account[j];
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
        v.push(owners[group_id].to_string());
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
    use leetcode_prelude::{assert_eq_sorted, vec2_string};
    assert_eq_sorted!(
        accounts_merge(vec2_string![
            ["John", "johnsmith@mail.com", "john00@mail.com"],
            ["John", "johnnybravo@mail.com"],
            ["John", "johnsmith@mail.com", "john_newyork@mail.com"],
            ["Mary", "mary@mail.com"]
        ]),
        vec2_string![
            [
                "John",
                "john00@mail.com",
                "john_newyork@mail.com",
                "johnsmith@mail.com"
            ],
            ["John", "johnnybravo@mail.com"],
            ["Mary", "mary@mail.com"]
        ]
    );
}
