// https://leetcode-cn.com/problems/car-pooling/

pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    let mut pairs = vec![];
    for trip in trips {
        pairs.push((trip[1], trip[0]));
        pairs.push((trip[2], -trip[0]));
    }
    pairs.sort_unstable();
    let mut max = 0;
    let mut count = 0;
    for pair in pairs {
        count += pair.1;
        max = max.max(count);
    }
    max <= capacity
}
// greedy
#[test]
fn test1_1094() {
    assert_eq!(car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 4), false);
    assert_eq!(car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 5), true);
    assert_eq!(car_pooling(vec![vec![2, 1, 5], vec![3, 5, 7]], 3), true);
    assert_eq!(
        car_pooling(vec![vec![3, 2, 7], vec![3, 7, 9], vec![8, 3, 9]], 11),
        true
    );
}
