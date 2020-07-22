// https://leetcode.com/problems/swim-in-rising-water/
pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// union_find depth_first_search heap binary_search
#[test]
#[ignore]
fn test3_778() {
    assert_eq!(swim_in_water(vec![vec![0, 2], vec![1, 3]]), 3);
    assert_eq!(
        swim_in_water(vec![
            vec![0, 1, 2, 3, 4],
            vec![24, 23, 22, 21, 5],
            vec![12, 13, 14, 15, 16],
            vec![11, 17, 18, 19, 20],
            vec![10, 9, 8, 7, 6]
        ]),
        16
    );
}
