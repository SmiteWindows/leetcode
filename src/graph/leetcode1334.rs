// https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/
pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
    todo!()
}
// graph
#[test]
#[ignore]
fn test1_1334() {
    assert_eq!(
        find_the_city(
            4,
            vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]],
            4
        ),
        3
    );
    assert_eq!(
        find_the_city(
            5,
            vec![
                vec![0, 1, 2],
                vec![0, 4, 8],
                vec![1, 2, 3],
                vec![1, 4, 2],
                vec![2, 3, 1],
                vec![3, 4, 1]
            ],
            2
        ),
        0
    );
}
