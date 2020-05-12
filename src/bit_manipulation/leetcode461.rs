// https://leetcode.com/problems/hamming-distance/
pub fn hamming_distance(x: i32, y: i32) -> i32 {
    let mut z = x ^ y;
    let mut distance = 0;
    for _ in 0..32 {
        distance += z & 1;
        z >>= 1;
    }
    distance
}
// bit_manipulation
#[test]
fn test1_461() {
    assert_eq!(hamming_distance(1, 4), 2);
}
