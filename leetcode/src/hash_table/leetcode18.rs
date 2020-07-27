// https://leetcode.com/problems/4sum/
pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    todo!()
}
// array two_pointers hash_table
#[test]
#[ignore]
fn test3_18() {
    use leetcode_prelude::{assert_eq_sorted, vec2};
    assert_eq_sorted!(
        four_sum(vec![1, 0, -1, 0, -2, 2], 0),
        vec2![[-1, 0, 0, 1], [-2, -1, 1, 2], [-2, 0, 0, 2]]
    );
}
