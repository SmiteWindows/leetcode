// https://leetcode-cn.com/problems/permutation-sequence/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn get_permutation(n: i32, k: i32) -> String {
    let n = n as usize;
    let mut k = k as usize - 1;
    let mut nums = (1..=n).collect::<Vec<usize>>();
    let mut factorial = vec![1];
    let mut prev = 1;
    for x in 1..n {
        prev *= x;
        factorial.push(prev);
    }
    let mut res = Vec::new();
    for i in 0..n {
        let index = k / factorial[n - 1 - i];
        res.push(nums.remove(index));
        k %= factorial[n - 1 - i];
    }
    res.into_iter().map(|x| (x as u8 + b'0') as char).collect()
}
// math backtracking
#[test]
fn test1_60() {
    assert_eq!(get_permutation(3, 3), "213".to_string());
    assert_eq!(get_permutation(4, 9), "2314".to_string());
}
