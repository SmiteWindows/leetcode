// https://leetcode-cn.com/problems/sort-characters-by-frequency/
// Runtime: 24 ms
// Memory Usage: 2.6 MB
use std::{cmp::Reverse, collections::HashMap};
pub fn frequency_sort(s: String) -> String {
    let mut s = s.chars().collect::<Vec<_>>();
    let mut hm: HashMap<char, usize> = HashMap::new();
    for &c in &s {
        *hm.entry(c).or_default() += 1;
    }
    s.sort_unstable_by_key(|&c| (Reverse(hm[&c]), Reverse(c)));
    s.into_iter().collect()
}
// hash_table heap
#[test]
fn test1_451() {
    // assert_eq!(frequency_sort(String::from("tree")), String::from("eert"));
    assert_eq!(frequency_sort(String::from("tree")), String::from("eetr"));
    assert_eq!(
        frequency_sort(String::from("cccaaa")),
        String::from("cccaaa")
    );
    // assert_eq!(frequency_sort(String::from("Aabb")), String::from("bbAa"));
    assert_eq!(frequency_sort(String::from("Aabb")), String::from("bbaA"));
}
