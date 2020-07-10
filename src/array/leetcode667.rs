// https://leetcode.com/problems/beautiful-arrangement-ii/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
    let mut k = k;
    let mut res = vec![1];
    let mut l = 2;
    let mut r = n;
    let mut forward = true;
    for _ in 1..n {
        if k > 1 {
            if forward {
                res.push(r);
                r -= 1;
            } else {
                res.push(l);
                l += 1;
            }
            forward = !forward;
            k -= 1;
        } else {
            if forward {
                res.push(l);
                l += 1;
            } else {
                res.push(r);
                r -= 1;
            }
        }
    }
    res
}
// array
#[test]
fn test1_667() {
    assert_eq!(construct_array(3, 1), vec![1, 2, 3]);
    assert_eq!(construct_array(3, 2), vec![1, 3, 2]);
}
