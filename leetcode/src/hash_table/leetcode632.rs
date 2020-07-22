// https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/
pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}
// hash_table two_pointers string
#[test]
#[ignore]
fn test2_632() {
    assert_eq!(
        smallest_range(vec![
            vec![4, 10, 15, 24, 26],
            vec![0, 9, 12, 20],
            vec![5, 18, 22, 30]
        ]),
        vec![20, 24]
    );
}
