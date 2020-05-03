// https://leetcode.com/problems/moving-stones-until-consecutive-ii/
pub fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {
    todo!()
}
// sliding_window array
#[test]
#[ignore]
fn test2_1040() {
    assert_eq!(num_moves_stones_ii(vec![7, 4, 9]), vec![1, 2]);
    assert_eq!(num_moves_stones_ii(vec![6, 5, 4, 3, 10]), vec![2, 3]);
    assert_eq!(
        num_moves_stones_ii(vec![100, 101, 104, 102, 103]),
        vec![0, 0]
    );
}
