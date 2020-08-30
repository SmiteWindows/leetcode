// https://leetcode-cn.com/problems/find-the-shortest-superstring/
pub fn shortest_superstring(a: Vec<String>) -> String {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_943() {
    assert_eq!(
        shortest_superstring(vec![
            String::from("alex"),
            String::from("loves"),
            String::from("leetcode")
        ]),
        String::from("alexlovesleetcode")
    );
    assert_eq!(
        shortest_superstring(vec![
            String::from("catg"),
            String::from("ctaagt"),
            String::from("gcta"),
            String::from("ttca"),
            String::from("atgcatc")
        ]),
        String::from("gctaagttcatgcatc")
    );
}
