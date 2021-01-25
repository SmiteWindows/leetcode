// https://leetcode-cn.com/problems/fruit-into-baskets/
// Runtime: 20 ms
// Memory Usage: 2.5 MB
use std::collections::HashMap;
pub fn total_fruit(tree: Vec<i32>) -> i32 {
    let mut last: HashMap<i32, usize> = HashMap::new();
    let mut start = 0;
    let mut res = 0;
    for (end, &t_end) in tree.iter().enumerate() {
        *last.entry(t_end).or_default() = end;
        if last.len() == 3 {
            let (index, key) = last.iter().map(|(&k, &v)| (v, k)).min().unwrap();
            start = index + 1;
            last.remove(&key);
        }
        res = res.max(end - start + 1);
    }
    res as i32
}
// two_pointers
#[test]
fn test1_904() {
    assert_eq!(total_fruit(vec![1, 2, 1]), 3);
    assert_eq!(total_fruit(vec![0, 1, 2, 2]), 3);
    assert_eq!(total_fruit(vec![1, 2, 3, 2, 2]), 4);
    assert_eq!(total_fruit(vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]), 5);
}
