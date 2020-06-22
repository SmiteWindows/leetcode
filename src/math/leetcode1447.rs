// https://leetcode.com/problems/simplified-fractions/
// Runtime: 48 ms
// Memory Usage: 2.4 MB
use std::collections::HashSet;
pub fn simplified_fractions(n: i32) -> Vec<String> {
    let mut hs = HashSet::new();
    for i in 2..=n {
        for j in 1..i {
            let k = gcd(i, j);
            hs.insert(format!("{}/{}", j / k, i / k));
        }
    }
    hs.into_iter().collect()
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a != 0 {
        let t = a;
        a = b % t;
        b = t;
    }
    b
}
// math
#[test]
fn test1_1447() {
    assert_eq!(simplified_fractions(2), vec![String::from("1/2")]);
    assert_eq!(
        simplified_fractions(3),
        vec![
            String::from("1/2"),
            String::from("1/3"),
            String::from("2/3")
        ]
    );
    assert_eq!(
        simplified_fractions(4),
        vec![
            String::from("1/2"),
            String::from("1/3"),
            String::from("1/4"),
            String::from("2/3"),
            String::from("3/4")
        ]
    );
    assert_eq!(simplified_fractions(1), vec![String::new()]);
}
