// https://leetcode-cn.com/problems/queries-on-a-permutation-with-key/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
    let mut v = (1..=m).collect::<Vec<i32>>();
    let mut res = Vec::new();
    for q in queries {
        let p = v.iter().position(|&x| x == q).unwrap();
        v.remove(p);
        v.insert(0, q);
        res.push(p as i32);
    }
    res
}
// array
#[test]
fn test1_1409() {
    assert_eq!(process_queries(vec![3, 1, 2, 1], 5), vec![2, 1, 2, 1]);
    assert_eq!(process_queries(vec![4, 1, 2, 2], 4), vec![3, 1, 2, 0]);
    assert_eq!(process_queries(vec![7, 5, 5, 8, 3], 8), vec![6, 5, 0, 7, 5]);
}
