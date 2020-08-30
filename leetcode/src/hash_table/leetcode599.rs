// https://leetcode-cn.com/problems/minimum-index-sum-of-two-lists/
// Runtime: 12 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    let mut hm = HashMap::new();
    let mut min = usize::MAX;
    let mut res = vec![];
    for (i, l1) in list1.iter().enumerate() {
        hm.insert(l1, i);
    }
    for (j, l2) in list2.iter().enumerate() {
        if let Some(i) = hm.get(l2) {
            let sum = i + j;
            if sum < min {
                min = sum;
                res.clear();
            }
            if sum == min {
                res.push(l2.to_string());
            }
        }
    }
    res
}
// hash_table
#[test]
fn test1_599() {
    assert_eq!(
        find_restaurant(
            vec![
                String::from("Shogun"),
                String::from("Tapioca Express"),
                String::from("Burger King"),
                String::from("KFC")
            ],
            vec![
                String::from("Piatti"),
                String::from("The Grill at Torrey Pines"),
                String::from("Hungry Hunter Steakhouse"),
                String::from("Shogun")
            ]
        ),
        vec![String::from("Shogun")]
    );
    assert_eq!(
        find_restaurant(
            vec![
                String::from("Shogun"),
                String::from("Tapioca Express"),
                String::from("Burger King"),
                String::from("KFC")
            ],
            vec![
                String::from("KFC"),
                String::from("Shogun"),
                String::from("Burger King")
            ]
        ),
        vec![String::from("Shogun")]
    );
}
