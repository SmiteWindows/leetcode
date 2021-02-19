// https://leetcode-cn.com/problems/counting-bits/
// Runtime: 4 ms
// Memory Usage: 2.6 MB
pub fn count_bits(num: i32) -> Vec<i32> {
    let num = num as usize;
    let mut ans = vec![0; num + 1];
    for i in 1..=num {
        ans[i] = ans[i >> 1] + (i as i32 & 1);
    }
    ans
}
// bit_manipulation dynamic_programming
#[test]
fn test2_338() {
    assert_eq!(count_bits(2), vec![0, 1, 1]);
    assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2]);
}
