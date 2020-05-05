// https://leetcode.com/problems/mini-parser/
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<Self>),
}

pub fn deserialize(s: String) -> NestedInteger {
    todo!()
}
// stack string
#[test]
#[ignore]
fn test1_385() {
    use self::NestedInteger::*;
    assert_eq!(deserialize(String::from("324")), Int(324));
    assert_eq!(
        deserialize(String::from("[123,[456,[789]]]")),
        List(vec![Int(123), List(vec![Int(456), List(vec![Int(789)])])])
    );
}
