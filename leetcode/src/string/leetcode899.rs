// https://leetcode-cn.com/problems/orderly-queue/
// Runtime: 4 ms
// Memory Usage: 2 MB
pub fn orderly_queue(s: String, k: i32) -> String {
    let mut s: Vec<char> = s.chars().collect();
    let n = s.len();
    if k > 1 {
        s.sort_unstable();
        s.into_iter().collect()
    } else {
        let mut res = s.to_vec();
        for i in 0..n {
            let mut t = Vec::new();
            for &sj in s.iter().take(n).skip(i) {
                t.push(sj);
            }
            for &sj in s.iter().take(i) {
                t.push(sj);
            }
            if t < res {
                res = t;
            }
        }
        res.into_iter().collect()
    }
}
// math string
#[test]
fn test2_899() {
    assert_eq!(orderly_queue("cba".to_string(), 1), "acb".to_string());
    assert_eq!(orderly_queue("baaca".to_string(), 3), "aaabc".to_string());
}
