// https://leetcode.com/problems/mini-parser/
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<Self>),
}

pub fn deserialize(s: String) -> NestedInteger {
    todo!()
}
