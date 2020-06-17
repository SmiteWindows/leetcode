// https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/
pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
    todo!()
}
// array sort
#[test]
#[ignore]
fn test2_1481() {
    assert_eq!(find_least_num_of_unique_ints(vec![5, 5, 4], 1), 1);
    assert_eq!(
        find_least_num_of_unique_ints(vec![4, 3, 1, 1, 3, 3, 2], 3),
        2
    );
}
