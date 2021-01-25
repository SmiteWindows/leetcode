// https://leetcode-cn.com/problems/kth-smallest-number-in-multiplication-table/
// Runtime: 12 ms
// Memory Usage: 1.9 MB
pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
    let mut lo = 1;
    let mut hi = m * n;
    while lo < hi {
        let mi = lo + (hi - lo) / 2;
        if count(mi, m, n) < k {
            lo = mi + 1;
        } else {
            hi = mi;
        }
    }
    lo
}

fn count(x: i32, m: i32, n: i32) -> i32 {
    let mut res = 0;
    for i in 1..=m {
        res += n.min(x / i);
    }
    res
}
// binary_search
#[test]
fn test1_668() {
    assert_eq!(find_kth_number(3, 3, 5), 3);
    assert_eq!(find_kth_number(2, 3, 6), 6);
}
