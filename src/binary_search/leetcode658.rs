// https://leetcode.com/problems/find-k-closest-elements/
pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    todo!()
}
// binary_search
#[test]
#[ignore]
fn test1_658() {
    assert_eq!(
        find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3),
        vec![1, 2, 3, 4]
    );
    assert_eq!(
        find_closest_elements(vec![1, 2, 3, 4, 5], 4, -1),
        vec![1, 2, 3, 4]
    );
}
