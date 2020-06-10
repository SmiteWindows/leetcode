// https://leetcode.com/problems/minimum-number-of-frogs-croaking/

use std::collections::HashMap;
pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
    if croak_of_frogs.len() % 5 != 0 {
        return -1;
    }
    let mut id = HashMap::new();
    for &c in &['c', 'r', 'o', 'a', 'k'] {
        id.insert(c, id.len());
    }
    let mut count = vec![0; 5];
    let mut frogs = 0;
    let mut res = 0;
    for c in croak_of_frogs.chars() {
        let i = id[&c];
        count[i] += 1;
        if i == 0 {
            frogs += 1;
            res = res.max(frogs);
        }
        if i > 0 {
            if count[i - 1] < count[i] {
                return -1;
            }
        }
        if i == 4 {
            frogs -= 1;
        }
    }
    res
}
// string
#[test]
fn test1_1419() {
    assert_eq!(min_number_of_frogs(String::from("croakcroak")), 1);
    assert_eq!(min_number_of_frogs(String::from("crcoakroak")), 2);
    assert_eq!(min_number_of_frogs(String::from("croakcrook")), -1);
    assert_eq!(min_number_of_frogs(String::from("croakcroa")), -1);
}
