// https://leetcode-cn.com/problems/smallest-sufficient-team/
pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
    todo!()
}
// bit_manipulation dynamic_programming
#[test]
#[ignore]
fn test1_1125() {
    use leetcode_prelude::{vec2_string, vec_string};
    assert_eq!(
        smallest_sufficient_team(
            vec_string!["java", "nodejs", "reactjs"],
            vec2_string![["java"], ["nodejs"], ["nodejs", "reactjs"]]
        ),
        vec![0, 2]
    );
    assert_eq!(
        smallest_sufficient_team(
            vec_string!["algorithms", "math", "java", "reactjs", "csharp", "aws"],
            vec2_string![
                ["algorithms", "math", "java"],
                ["algorithms", "math", "reactjs"],
                ["java", "csharp", "aws"],
                ["reactjs", "csharp"],
                ["csharp", "math"],
                ["aws", "java"]
            ]
        ),
        vec![1, 2]
    );
}
