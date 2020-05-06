// https://leetcode.com/problems/find-duplicate-file-in-system/
pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
    todo!()
}
// hash_table string
#[test]
#[ignore]
fn test1_609() {
    assert_eq!(
        find_duplicate(vec![
            String::from("root/a 1.txt(abcd) 2.txt(efgh)"),
            String::from("root/c 3.txt(abcd)"),
            String::from("root/c/d 4.txt(efgh)"),
            String::from("root 4.txt(efgh)")
        ]),
        vec![
            vec![
                String::from("root/a/2.txt"),
                String::from("root/c/d/4.txt"),
                String::from("root/4.txt")
            ],
            vec![String::from("root/a/1.txt"), String::from("root/c/3.txt")]
        ]
    );
}
