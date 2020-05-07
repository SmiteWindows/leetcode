// https://leetcode.com/problems/flood-fill/
pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
    todo!()
}
// depth_first_search
#[test]
#[ignore]
fn test1_733() {
    assert_eq!(
        flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
        vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
    );
}
