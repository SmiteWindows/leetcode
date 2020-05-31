// https://leetcode.com/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts/
pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_1465() {
    assert_eq!(max_area(5, 4, vec![1, 2, 4], vec![1, 3]), 4);
    assert_eq!(max_area(5, 4, vec![3, 1], vec![1]), 6);
    assert_eq!(max_area(5, 4, vec![3], vec![3]), 9);
}
