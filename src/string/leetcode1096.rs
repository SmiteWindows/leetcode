// https://leetcode.com/problems/brace-expansion-ii/
pub fn brace_expansion_ii(expression: String) -> Vec<String> {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_1096() {
    assert_eq!(
        brace_expansion_ii(String::from("{a,b}{c,{d,e}}")),
        vec![
            String::from("ac"),
            String::from("ad"),
            String::from("ae"),
            String::from("bc"),
            String::from("bd"),
            String::from("be")
        ]
    );
    assert_eq!(
        brace_expansion_ii(String::from("{{a,z},a{b,c},{ab,z}}")),
        vec![
            String::from("a"),
            String::from("ab"),
            String::from("ac"),
            String::from("z")
        ]
    );
}
