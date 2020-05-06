// https://leetcode.com/problems/cheapest-flights-within-k-stops/
pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    todo!()
}
// heap dynamic_programming breadth_first_search
#[test]
#[ignore]
fn test2_787() {
    assert_eq!(
        find_cheapest_price(
            3,
            vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
            0,
            2,
            1
        ),
        200
    );
    assert_eq!(
        find_cheapest_price(
            3,
            vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
            0,
            2,
            0
        ),
        500
    );
}
