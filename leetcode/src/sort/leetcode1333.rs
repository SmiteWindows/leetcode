// https://leetcode-cn.com/problems/filter-restaurants-by-vegan-friendly-price-and-distance/
// Runtime: 8 ms
// Memory Usage: 3 MB
use std::cmp::Reverse;
pub fn filter_restaurants(
    restaurants: Vec<Vec<i32>>,
    vegan_friendly: i32,
    max_price: i32,
    max_distance: i32,
) -> Vec<i32> {
    let mut pairs = restaurants
        .into_iter()
        .filter_map(|r| {
            if (vegan_friendly == 0 || r[2] == 1) && r[3] <= max_price && r[4] <= max_distance {
                Some((Reverse(r[1]), Reverse(r[0])))
            } else {
                None
            }
        })
        .collect::<Vec<(Reverse<i32>, Reverse<i32>)>>();
    pairs.sort_unstable();
    pairs.into_iter().map(|p| (p.1).0).collect()
}
// sort array
#[test]
fn test1_1333() {
    assert_eq!(
        filter_restaurants(
            vec![
                vec![1, 4, 1, 40, 10],
                vec![2, 8, 0, 50, 5],
                vec![3, 8, 1, 30, 4],
                vec![4, 10, 0, 10, 3],
                vec![5, 1, 1, 15, 1]
            ],
            1,
            50,
            10
        ),
        vec![3, 1, 5]
    );
    assert_eq!(
        filter_restaurants(
            vec![
                vec![1, 4, 1, 40, 10],
                vec![2, 8, 0, 50, 5],
                vec![3, 8, 1, 30, 4],
                vec![4, 10, 0, 10, 3],
                vec![5, 1, 1, 15, 1]
            ],
            0,
            50,
            10
        ),
        vec![4, 3, 2, 1, 5]
    );
    assert_eq!(
        filter_restaurants(
            vec![
                vec![1, 4, 1, 40, 10],
                vec![2, 8, 0, 50, 5],
                vec![3, 8, 1, 30, 4],
                vec![4, 10, 0, 10, 3],
                vec![5, 1, 1, 15, 1]
            ],
            0,
            30,
            3
        ),
        vec![4, 5]
    );
}
