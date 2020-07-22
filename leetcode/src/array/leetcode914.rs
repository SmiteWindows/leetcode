// https://leetcode.com/problems/x-of-a-kind-in-a-deck-of-cards/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashMap;
pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
    let mut hm = HashMap::new();
    let mut max = 0;
    for x in deck {
        *hm.entry(x).or_default() += 1;
    }
    for &v in hm.values() {
        max = gcd(max, v);
    }
    max >= 2
}

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}
// math array
#[test]
fn test2_914() {
    assert_eq!(has_groups_size_x(vec![1, 2, 3, 4, 4, 3, 2, 1]), true);
    assert_eq!(has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 3, 3]), false);
    assert_eq!(has_groups_size_x(vec![1]), false);
    assert_eq!(has_groups_size_x(vec![1, 1]), true);
    assert_eq!(has_groups_size_x(vec![1, 1, 2, 2, 2, 2]), true);
}
