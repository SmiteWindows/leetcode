// https://leetcode-cn.com/problems/text-justification/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::collections::VecDeque;
pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let mut queue = VecDeque::new();
    let max_width = max_width as usize;
    let mut width = 0;
    let mut res = vec![];
    for word in words {
        if width + queue.len() + word.len() > max_width as usize {
            res.push(make_a_line(&mut queue, max_width, width));
            width = word.len();
            queue.push_back(word);
        } else {
            width += word.len();
            queue.push_back(word);
        }
    }
    res.push(make_the_last_line(&mut queue, max_width, width));
    res
}

fn make_a_line(queue: &mut VecDeque<String>, max_width: usize, width: usize) -> String {
    let mut line = "".to_string();
    let mut space = max_width - width;
    while let Some(s) = queue.pop_front() {
        line += &s;
        let d = if !queue.is_empty() {
            space / queue.len() + if space % queue.len() != 0 { 1 } else { 0 }
        } else {
            space
        };
        space -= d;
        for _ in 0..d {
            line.push(' ');
        }
    }
    line
}

fn make_the_last_line(queue: &mut VecDeque<String>, max_width: usize, width: usize) -> String {
    let mut line = "".to_string();
    let mut space = max_width - width;
    while let Some(s) = queue.pop_front() {
        line += &s;
        if queue.is_empty() {
            for _ in 0..space {
                line.push(' ');
            }
        } else {
            line.push(' ');
            space -= 1;
        }
    }
    line
}
// string
#[test]
fn test1_68() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        full_justify(
            vec_string![
                "This",
                "is",
                "an",
                "example",
                "of",
                "text",
                "justification."
            ],
            16
        ),
        vec_string!["This    is    an", "example  of text", "justification.  "]
    );
    assert_eq!(
        full_justify(
            vec_string!["What", "must", "be", "acknowledgment", "shall", "be"],
            16
        ),
        vec_string!["What   must   be", "acknowledgment  ", "shall be        "]
    );
    assert_eq!(
        full_justify(
            vec_string![
                "Science",
                "is",
                "what",
                "we",
                "understand",
                "well",
                "enough",
                "to",
                "explain",
                "to",
                "a",
                "computer.",
                "Art",
                "is",
                "everything",
                "else",
                "we",
                "do"
            ],
            20
        ),
        vec_string![
            "Science  is  what we",
            "understand      well",
            "enough to explain to",
            "a  computer.  Art is",
            "everything  else  we",
            "do                  "
        ]
    );
}
