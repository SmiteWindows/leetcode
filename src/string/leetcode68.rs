// https://leetcode.com/problems/text-justification/
pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_68() {
    assert_eq!(
        full_justify(
            vec![
                String::from("This"),
                String::from("is"),
                String::from("an"),
                String::from("example"),
                String::from("of"),
                String::from("text"),
                String::from("justification.")
            ],
            16
        ),
        vec![
            String::from("This    is    an"),
            String::from("example  of text"),
            String::from("justification.  ")
        ]
    );
    assert_eq!(
        full_justify(
            vec![
                String::from("What"),
                String::from("must"),
                String::from("be"),
                String::from("acknowledgment"),
                String::from("shall"),
                String::from("be")
            ],
            16
        ),
        vec![
            String::from("What   must   be"),
            String::from("acknowledgment  "),
            String::from("shall be        ")
        ]
    );
    assert_eq!(
        full_justify(
            vec![
                String::from("Science"),
                String::from("is"),
                String::from("what"),
                String::from("we"),
                String::from("understand"),
                String::from("well"),
                String::from("enough"),
                String::from("to"),
                String::from("explain"),
                String::from("to"),
                String::from("a"),
                String::from("computer."),
                String::from("Art"),
                String::from("is"),
                String::from("everything"),
                String::from("else"),
                String::from("we"),
                String::from("do")
            ],
            20
        ),
        vec![
            String::from("Science  is  what we"),
            String::from("understand      well"),
            String::from("enough to explain to"),
            String::from("a  computer.  Art is"),
            String::from("everything  else  we"),
            String::from("do                  ")
        ]
    );
}
