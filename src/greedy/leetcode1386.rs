// https://leetcode.com/problems/cinema-seat-allocation/
pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// greedy array
#[test]
#[ignore]
fn test1_1386() {
    assert_eq!(
        max_number_of_families(
            3,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 8],
                vec![2, 6],
                vec![3, 1],
                vec![3, 10]
            ]
        ),
        4
    );
    assert_eq!(
        max_number_of_families(2, vec![vec![2, 1], vec![1, 8], vec![2, 6]]),
        2
    );
    assert_eq!(
        max_number_of_families(4, vec![vec![4, 3], vec![1, 4], vec![4, 6], vec![1, 7]]),
        4
    );
}
