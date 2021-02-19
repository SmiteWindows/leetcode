// https://leetcode-cn.com/problems/car-pooling/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    let mut pairs = Vec::new();
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
    use leetcode_prelude::vec2;
    assert_eq!(car_pooling(vec2![[2, 1, 5], [3, 3, 7]], 4), false);
    assert_eq!(car_pooling(vec2![[2, 1, 5], [3, 3, 7]], 5), true);
    assert_eq!(car_pooling(vec2![[2, 1, 5], [3, 5, 7]], 3), true);
    assert_eq!(
        car_pooling(vec2![[3, 2, 7], [3, 7, 9], [8, 3, 9]], 11),
        true
    );
}
