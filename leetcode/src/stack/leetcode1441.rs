// https://leetcode-cn.com/problems/build-an-array-with-stack-operations/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
    let mut res = Vec::new();
    let mut curr = 1;
    for i in target {
        if i != curr {
            res.extend(["Push", "Pop"].repeat((i - curr) as usize));
        }
        res.push("Push");
        curr = i + 1;
    }
    res.into_iter().map(|a| a.to_string()).collect()
}
// stack
#[test]
fn test1_1441() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        build_array(vec![1, 3], 3),
        vec_string!["Push", "Push", "Pop", "Push"]
    );
    assert_eq!(
        build_array(vec![1, 2, 3], 3),
        vec_string!["Push", "Push", "Push"]
    );
    assert_eq!(build_array(vec![1, 2], 4), vec_string!["Push", "Push"]);
    assert_eq!(
        build_array(vec![2, 3, 4], 4),
        vec_string!["Push", "Pop", "Push", "Push", "Push"]
    );
}
