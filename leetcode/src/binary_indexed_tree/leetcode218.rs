// https://leetcode.com/problems/the-skyline-problem/
pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    todo!()
}
// divide_and_conquer heap binary_indexed_tree segment_tree line_sweep
#[test]
#[ignore]
fn test4_218() {
    assert_eq!(
        get_skyline(vec![
            vec![2, 9, 10],
            vec![3, 7, 15],
            vec![5, 12, 12],
            vec![15, 20, 10],
            vec![19, 24, 8]
        ]),
        vec![
            vec![2, 10],
            vec![3, 15],
            vec![7, 12],
            vec![12, 0],
            vec![15, 10],
            vec![20, 8],
            vec![24, 0]
        ]
    );
}
