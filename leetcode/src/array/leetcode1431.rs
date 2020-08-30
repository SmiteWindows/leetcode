// https://leetcode-cn.com/problems/kids-with-the-greatest-number-of-candies/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max = *candies.iter().max().unwrap();
    candies
        .into_iter()
        .map(|x| x + extra_candies >= max)
        .collect()
}
// array
#[test]
fn test1_1431() {
    assert_eq!(
        kids_with_candies(vec![2, 3, 5, 1, 3], 3),
        vec![true, true, true, false, true]
    );
    assert_eq!(
        kids_with_candies(vec![4, 2, 1, 1, 2], 1),
        vec![true, false, false, false, false]
    );
    assert_eq!(
        kids_with_candies(vec![12, 1, 12], 10),
        vec![true, false, true]
    );
}
