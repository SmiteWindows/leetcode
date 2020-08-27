// https://leetcode.com/problems/largest-palindrome-product/
// Runtime: 196 ms
// Memory Usage: 2.1 MB
pub fn largest_palindrome(n: i32) -> i32 {
    if n == 1 {
        return 9;
    }
    let max = 10u64.pow(n as u32) - 1;
    for i in (0..max).rev() {
        let left: String = i.to_string();
        let right: String = i.to_string().chars().rev().collect();
        let palindrome = format!("{}{}", left, right);
        if let Ok(value) = palindrome.parse::<u64>() {
            let mut j = max;
            while j * j >= value {
                if value % j == 0 {
                    return (value % 1337) as i32;
                }
                j -= 1;
            }
        }
    }
    0
}
#[test]
fn test479(){
    assert_eq!(largest_palindrome(2),987);
}