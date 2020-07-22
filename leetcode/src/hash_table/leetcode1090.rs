// https://leetcode.com/problems/largest-values-from-labels/
// Runtime: 4 ms
// Memory Usage: 2.8 MB
use std::collections::HashMap;
type Pair = (i32, i32);
pub fn largest_vals_from_labels(
    values: Vec<i32>,
    labels: Vec<i32>,
    num_wanted: i32,
    use_limit: i32,
) -> i32 {
    let n = values.len();
    let use_limit = use_limit as usize;
    let mut num_wanted = num_wanted as usize;
    let mut pairs = values
        .into_iter()
        .zip(labels.into_iter())
        .collect::<Vec<Pair>>();
    pairs.sort_unstable();
    let mut hm: HashMap<i32, usize> = HashMap::new();
    let mut res = 0;
    for i in (0..n).rev() {
        let count = hm.entry(pairs[i].1).or_default();
        if *count < use_limit {
            *count += 1;
            res += pairs[i].0;
            num_wanted -= 1;
        }
        if num_wanted == 0 {
            break;
        }
    }
    res
}
// hash_table greedy
#[test]
fn test1_1090() {
    assert_eq!(
        largest_vals_from_labels(vec![5, 4, 3, 2, 1], vec![1, 1, 2, 2, 3], 3, 1),
        9
    );
    assert_eq!(
        largest_vals_from_labels(vec![5, 4, 3, 2, 1], vec![1, 3, 3, 3, 2], 3, 2),
        12
    );
    assert_eq!(
        largest_vals_from_labels(vec![9, 8, 8, 7, 6], vec![0, 0, 0, 1, 1], 3, 1),
        16
    );
    assert_eq!(
        largest_vals_from_labels(vec![9, 8, 8, 7, 6], vec![0, 0, 0, 1, 1], 3, 2),
        24
    );
}
