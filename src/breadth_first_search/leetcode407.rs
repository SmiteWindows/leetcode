// https://leetcode.com/problems/trapping-rain-water-ii/
pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// heap breadth_first_search
#[test]
#[ignore]
fn test2_407() {
    assert_eq!(
        trap_rain_water(vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 1, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1]
        ]),
        4
    );
}
