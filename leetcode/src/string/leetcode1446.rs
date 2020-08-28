// https://leetcode.com/problems/consecutive-characters/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn max_power(s: String) -> i32 {
    s.bytes()
        .fold((0, 0, b'a'), |(mut count, max, last), l| {
            if l != last {
                count = 0;
            }
            (count + 1, max.max(count + 1), l)
        })
        .1
}
// string
#[test]
fn test1_1446() {
    assert_eq!(max_power(String::from("leetcode")), 2);
    assert_eq!(max_power(String::from("abbcccddddeeeeedcba")), 5);
    assert_eq!(max_power(String::from("triplepillooooow")), 5);
    assert_eq!(max_power(String::from("hooraaaaaaaaaaay")), 11);
    assert_eq!(max_power(String::from("tourist")), 1);
}
