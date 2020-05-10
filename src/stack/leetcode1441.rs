// https://leetcode.com/problems/build-an-array-with-stack-operations/
pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
    todo!()
}
// stack
#[test]
#[ignore]
fn test1_1441() {
    assert_eq!(
        build_array(vec![1, 3], 3),
        vec![
            String::from("Push"),
            String::from("Push"),
            String::from("Pop"),
            String::from("Push")
        ]
    );
    assert_eq!(
        build_array(vec![1, 2, 3], 3),
        vec![
            String::from("Push"),
            String::from("Push"),
            String::from("Push")
        ]
    );
    assert_eq!(
        build_array(vec![1, 2], 4),
        vec![String::from("Push"), String::from("Push")]
    );
    assert_eq!(
        build_array(vec![2, 3, 4], 4),
        vec![
            String::from("Push"),
            String::from("Pop"),
            String::from("Push"),
            String::from("Push"),
            String::from("Push")
        ]
    );
}
