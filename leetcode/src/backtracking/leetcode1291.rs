// https://leetcode-cn.com/problems/sequential-digits/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    let mut res = vec![];
    for i in 1..10 {
        dfs(i, i, &mut res, low, high);
    }
    res.sort_unstable();
    res
}

fn dfs(last_digit: i32, cur: i32, res: &mut Vec<i32>, low: i32, high: i32) {
    if cur >= low && cur <= high {
        res.push(cur);
    }
    if cur >= high {
        return;
    }
    if last_digit < 9 {
        dfs(last_digit + 1, cur * 10 + last_digit + 1, res, low, high);
    }
}
// backtracking
#[test]
fn test1_1291() {
    assert_eq!(sequential_digits(100, 300), vec![123, 234]);
    assert_eq!(
        sequential_digits(1000, 13000),
        vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]
    );
}
