// https://leetcode.com/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons/
pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1420() {
    assert_eq!(num_of_arrays(2, 3, 1), 6);
    assert_eq!(num_of_arrays(5, 2, 3), 0);
    assert_eq!(num_of_arrays(9, 1, 1), 1);
    assert_eq!(num_of_arrays(50, 100, 25), 34549172);
    assert_eq!(num_of_arrays(37, 17, 7), 418930126);
}
