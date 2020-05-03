// https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/
pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    todo!()
}
// array
#[test]
#[ignore]
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
