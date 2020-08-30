// https://leetcode-cn.com/problems/koko-eating-bananas/
// Runtime: 8 ms
// Memory Usage: 2.3 MB
pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut l = 1;
    let mut r = 1_000_000_000;
    while l < r {
        let m = (l + r) / 2;
        let mut sum = 0;
        for p in &piles {
            sum += if p % m == 0 { p / m } else { p / m + 1 };
        }
        if sum > h {
            l = m + 1;
        } else {
            r = m;
        }
    }
    l
}
// binary_search
#[test]
fn test1_875() {
    assert_eq!(min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
    assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
}
