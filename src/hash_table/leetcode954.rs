// https://leetcode.com/problems/array-of-doubled-pairs/
pub fn can_reorder_doubled(a: Vec<i32>) -> bool {
    todo!()
}
// hash_table array
#[test]
#[ignore]
fn test1_954() {
    assert_eq!(can_reorder_doubled(vec![3, 1, 3, 6]), false);
    assert_eq!(can_reorder_doubled(vec![2, 1, 2, 6]), false);
    assert_eq!(can_reorder_doubled(vec![4, -2, 2, -4]), true);
    assert_eq!(can_reorder_doubled(vec![1, 2, 4, 16, 8, 4]), false);
}
