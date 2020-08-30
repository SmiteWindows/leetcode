// https://leetcode-cn.com/problems/groups-of-special-equivalent-strings/
// Runtime: 4 ms
// Memory Usage: 2.4 MB
use std::collections::{BTreeMap, HashSet};
pub fn num_special_equiv_groups(a: Vec<String>) -> i32 {
    let mut hs = HashSet::new();
    for s in a {
        hs.insert(Count::new(s));
    }
    hs.len() as i32
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Count {
    even: BTreeMap<char, usize>,
    odd: BTreeMap<char, usize>,
}

impl Count {
    fn new(s: String) -> Self {
        let mut even: BTreeMap<char, usize> = BTreeMap::new();
        let mut odd: BTreeMap<char, usize> = BTreeMap::new();
        for (i, c) in s.char_indices() {
            if i % 2 == 0 {
                *even.entry(c).or_default() += 1;
            } else {
                *odd.entry(c).or_default() += 1;
            }
        }
        Self { even, odd }
    }
}
// string
#[test]
fn test1_893() {
    assert_eq!(
        num_special_equiv_groups(vec![
            String::from("abcd"),
            String::from("cdab"),
            String::from("cbad"),
            String::from("xyzz"),
            String::from("zzxy"),
            String::from("zzyx")
        ]),
        3
    );
}
