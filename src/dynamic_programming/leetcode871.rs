// https://leetcode.com/problems/minimum-number-of-refueling-stops/
pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// heap dynamic_programming
#[test]
#[ignore]
fn test2_871() {
    assert_eq!(min_refuel_stops(1, 1, vec![vec![]]), 0);
    assert_eq!(min_refuel_stops(100, 1, vec![vec![10, 100]]), -1);
    assert_eq!(
        min_refuel_stops(
            100,
            10,
            vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]]
        ),
        2
    );
}
