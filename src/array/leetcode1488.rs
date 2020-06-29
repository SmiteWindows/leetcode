// https://leetcode.com/problems/avoid-flood-in-the-city/
// Runtime: 68 ms
// Memory Usage: 5.1 MB
use std::collections::{BTreeSet, HashMap};
pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
    let n = rains.len();
    let mut lakes = HashMap::new();
    let mut dry_days = BTreeSet::new();
    let mut res = vec![-1; n];
    for i in 0..n {
        if rains[i] == 0 {
            dry_days.insert(i);
        } else {
            if let Some(j) = lakes.insert(rains[i], i) {
                let mut pick_a_day = None;
                for &k in &dry_days {
                    if j < k {
                        pick_a_day = Some(k);
                        break;
                    }
                }
                if let Some(k) = pick_a_day {
                    res[k] = rains[i];
                    dry_days.remove(&k);
                } else {
                    return vec![];
                }
            }
        }
    }
    let (any_lake, _) = lakes.into_iter().next().unwrap();
    for i in dry_days {
        res[i] = any_lake;
    }
    res
}
// array hash_table
#[test]
fn test1_1488() {
    assert_eq!(avoid_flood(vec![1, 2, 3, 4]), vec![-1, -1, -1, -1]);
    assert_eq!(
        avoid_flood(vec![1, 2, 0, 0, 2, 1]),
        vec![-1, -1, 2, 1, -1, -1]
    );
    assert_eq!(avoid_flood(vec![1, 2, 0, 1, 2]), vec![]);
    // assert_eq!(avoid_flood(vec![69, 0, 0, 0, 69]), vec![-1, 69, 1, 1, -1]);
    assert_eq!(avoid_flood(vec![69, 0, 0, 0, 69]), vec![-1, 69, 69, 69, -1]);
    assert_eq!(avoid_flood(vec![10, 20, 20]), vec![]);
}
