// https://leetcode-cn.com/problems/water-bottles/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
    let mut full = num_bottles;
    let mut empty = 0;
    let mut res = 0;
    while full > 0 {
        res += full;
        empty += full;
        full = empty / num_exchange;
        empty %= num_exchange;
    }
    res
}
// greedy
#[test]
fn test1_1518() {
    assert_eq!(num_water_bottles(9, 3), 13);
    assert_eq!(num_water_bottles(15, 4), 19);
    assert_eq!(num_water_bottles(5, 5), 6);
    assert_eq!(num_water_bottles(2, 3), 2);
}
