// https://leetcode-cn.com/problems/friends-of-appropriate-ages/
// Runtime: 12 ms
// Memory Usage: 2.3 MB
use std::collections::HashMap;
pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
    let mut hm: HashMap<i32, i32> = HashMap::new();
    for age in ages {
        *hm.entry(age).or_default() += 1;
    }
    let mut res = 0;
    for (&a, v) in &hm {
        for (&b, u) in &hm {
            if !(b > a || 2 * b <= a + 14) {
                res += v * u;
                if a == b {
                    res -= v;
                }
            }
        }
    }
    res
}
// array
#[test]
fn test1_825() {
    assert_eq!(num_friend_requests(vec![16, 16]), 2);
    assert_eq!(num_friend_requests(vec![16, 17, 18]), 2);
    assert_eq!(num_friend_requests(vec![20, 30, 100, 110, 120]), 3);
}
