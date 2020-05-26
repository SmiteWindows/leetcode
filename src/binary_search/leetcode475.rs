// https://leetcode.com/problems/heaters/
// Runtime: 8 ms
// Memory Usage: 2.3 MB
pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
    let mut heaters = heaters;
    heaters.sort_unstable();
    let n = heaters.len();
    houses
        .iter()
        .map(|h| (h, heaters.binary_search(&h)))
        .map(|(h, min_dist)| match min_dist {
            Ok(_) => 0,
            Err(i) => match i {
                0 => heaters[i] - h,
                x if x == n => h - heaters[i - 1],
                _ => (heaters[i] - h).min(h - heaters[i - 1]),
            },
        })
        .max()
        .unwrap_or(std::i32::MAX)
}
// binary_search
#[test]
fn test1_475() {
    assert_eq!(find_radius(vec![1, 2, 3], vec![2]), 1);
    assert_eq!(find_radius(vec![1, 2, 3, 4], vec![1, 4]), 1);
}
