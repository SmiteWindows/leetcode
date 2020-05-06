// https://leetcode.com/problems/number-of-burgers-with-no-waste-of-ingredients/
pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
    todo!()
}
// greedy math
#[test]
#[ignore]
fn test2_1276() {
    assert_eq!(num_of_burgers(16, 7), vec![1, 6]);
    assert_eq!(num_of_burgers(17, 4), vec![]);
    assert_eq!(num_of_burgers(4, 17), vec![]);
    assert_eq!(num_of_burgers(0, 0), vec![0, 0]);
    assert_eq!(num_of_burgers(2, 1), vec![0, 1]);
}
