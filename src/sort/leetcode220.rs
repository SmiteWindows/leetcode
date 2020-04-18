// https://leetcode.com/problems/contains-duplicate-iii/
pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
    todo!()
}
// sort ordered_map
#[test]
#[ignore]
fn test2_220() {
    assert_eq!(
        true,
        contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0)
    );
    assert_eq!(
        true,
        contains_nearby_almost_duplicate(vec![1, 0, 1, 1], 1, 2)
    );
    assert_eq!(
        false,
        contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3)
    );
}
