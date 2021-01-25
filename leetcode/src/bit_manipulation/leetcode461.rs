// https://leetcode-cn.com/problems/hamming-distance/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn hamming_distance(x: i32, y: i32) -> i32 {
    let mut z = x ^ y;
    let mut distance = 0;
    while z != 0 {
        distance += 1;
        z = z & (z - 1);
    }
    distance
}
// bit_manipulation
#[test]
fn test1_461() {
    assert_eq!(hamming_distance(1, 4), 2);
}
