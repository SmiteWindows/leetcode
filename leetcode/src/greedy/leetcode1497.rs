// https://leetcode.com/problems/check-if-array-pairs-are-divisible-by-k/
// Runtime: 16 ms
// Memory Usage: 3.5 MB
pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
    let mut count = vec![0; k as usize];
    for x in arr {
        let y = x % k;
        let y = if y < 0 { y + k } else { y };
        count[y as usize] += 1;
    }
    if count[0] % 2 != 0 {
        return false;
    }
    let k = k as usize;
    for i in 1..k {
        if count[i] != count[k - i] {
            return false;
        }
    }
    true
}
// array math greedy
#[test]
fn test3_1497() {
    assert_eq!(can_arrange(vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9], 5), true);
    assert_eq!(can_arrange(vec![1, 2, 3, 4, 5, 6], 7), true);
    assert_eq!(can_arrange(vec![1, 2, 3, 4, 5, 6], 10), false);
    assert_eq!(can_arrange(vec![-10, 10], 2), true);
    assert_eq!(can_arrange(vec![-1, 1, -2, 2, -3, 3, -4, 4], 3), true);
}
