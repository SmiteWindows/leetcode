// https://leetcode.com/problems/plus-one/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut sum = vec![];
    let mut carry = 0;
    for (i, &d) in digits.iter().rev().enumerate() {
        let x = if i == 0 { d + 1 } else { d + carry };
        carry = x / 10;
        sum.insert(0, x % 10);
    }
    if carry != 0 {
        sum.insert(0, carry);
    }
    sum
}
// array
#[test]
fn test1_66() {
    assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
}
