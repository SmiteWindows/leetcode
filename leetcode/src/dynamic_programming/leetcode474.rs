// https://leetcode-cn.com/problems/ones-and-zeroes/
// Runtime: 308 ms
// Memory Usage: 21.3 MB
use std::collections::HashMap;
pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let mut count = strs
        .iter()
        .map(|s| {
            (
                s.chars().filter(|&c| c == '0').count() as i32,
                s.chars().filter(|&c| c == '1').count() as i32,
            )
        })
        .collect::<Vec<_>>();
    count.sort_unstable();
    let mut memo = HashMap::new();
    dp(0, m, n, &mut memo, &count, strs.len()) as i32
}

fn dp(
    start: usize,
    zero: i32,
    one: i32,
    memo: &mut HashMap<(usize, i32, i32), usize>,
    count: &[(i32, i32)],
    n: usize,
) -> usize {
    if start == n {
        return 0;
    }
    if let Some(&res) = memo.get(&(start, zero, one)) {
        return res;
    }
    let skip = dp(start + 1, zero, one, memo, count, n);
    let res = if zero >= count[start].0 && one >= count[start].1 {
        let keep = 1 + dp(
            start + 1,
            zero - count[start].0,
            one - count[start].1,
            memo,
            count,
            n,
        );
        skip.max(keep)
    } else {
        skip
    };
    memo.insert((start, zero, one), res);
    res
}
// dynamic_programming
#[test]
fn test1_474() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        find_max_form(vec_string!["10", "0001", "111001", "1", "0"], 5, 3),
        4
    );
    assert_eq!(find_max_form(vec_string!["10", "0", "1"], 1, 1), 2);
}
