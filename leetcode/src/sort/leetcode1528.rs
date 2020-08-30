// https://leetcode-cn.com/problems/shuffle-string/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let mut result = s.clone();

    for (i, &index) in indices.iter().enumerate() {
        let index = index as usize;
        unsafe {
            result.as_bytes_mut()[index] = s.as_bytes()[i];
        }
    }

    result
    // let n = s.len();
    // let s = s.chars().collect::<Vec<_>>();
    // let mut v = vec![' '; n];
    // for i in 0..n {
    //     v[indices[i] as usize] = s[i];
    // }
    // v.into_iter().collect()
}
// sort
#[test]
fn test1_1528() {
    assert_eq!(
        restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]),
        "leetcode".to_string()
    );
    assert_eq!(
        restore_string("abc".to_string(), vec![0, 1, 2]),
        "abc".to_string()
    );
    assert_eq!(
        restore_string("aiohn".to_string(), vec![3, 1, 4, 2, 0]),
        "nihao".to_string()
    );
    assert_eq!(
        restore_string("aaiougrt".to_string(), vec![4, 0, 2, 6, 7, 3, 1, 5]),
        "arigatou".to_string()
    );
    assert_eq!(
        restore_string("art".to_string(), vec![1, 0, 2]),
        "rat".to_string()
    );
}
