// https://leetcode.com/problems/number-of-boomerangs/
// Runtime: 44 ms
// Memory Usage: 2.1 MB
use std::collections::HashMap;
pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
    fn distance_square(a: &[i32], b: &[i32]) -> i32 {
        (a[0] - b[0]) * (a[0] - b[0]) + (a[1] - b[1]) * (a[1] - b[1])
    }

    let n = points.len();
    let mut hm: HashMap<i32, i32> = HashMap::new();
    let mut sum = 0;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let a = &points[i];
            let b = &points[j];
            let distance_square = distance_square(a, b);
            *hm.entry(distance_square).or_default() += 1;
        }
        for &value in hm.values() {
            if value > 1 {
                sum += value * (value - 1);
            }
        }
        hm.clear();
    }
    sum
}
// hash_table
#[test]
fn test1_447() {
    assert_eq!(
        number_of_boomerangs(vec![vec![0, 0], vec![1, 0], vec![2, 0]]),
        2
    );
}
