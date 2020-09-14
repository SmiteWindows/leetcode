// https://leetcode-cn.com/problems/remove-k-digits/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn remove_kdigits(num: String, k: i32) -> String {
    let mut k = k;
    let mut stack = vec![];
    for c in num.chars() {
        while let Some(&top) = stack.last() {
            if k > 0 && top > c {
                stack.pop();
                k -= 1;
            } else {
                break;
            }
        }
        stack.push(c);
    }
    while k != 0 {
        stack.pop();
        k -= 1;
    }
    let res = stack.into_iter().skip_while(|&c| c == '0').collect();
    if res == "" {
        "0".to_string()
    } else {
        res
    }
}
// stack greedy
#[test]
fn test1_402() {
    assert_eq!(remove_kdigits("1432219".to_string(), 3), "1219".to_string());
    assert_eq!(remove_kdigits("10200".to_string(), 1), "200".to_string());
    assert_eq!(remove_kdigits("10".to_string(), 2), "0".to_string());
}
