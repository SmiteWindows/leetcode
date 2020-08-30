// https://leetcode-cn.com/problems/exclusive-time-of-functions/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
    let n = n as usize;
    let mut stack = vec![];
    let mut res = vec![0; n];
    let mut prev = 0;
    for log in logs {
        let mut it = log.split(':');
        let id = it.next().unwrap().parse::<usize>().unwrap();
        let action: Action = if it.next().unwrap() == "start" {
            Action::Start
        } else {
            Action::End
        };
        let val = it.next().unwrap().parse::<i32>().unwrap();
        match action {
            Action::Start => {
                if let Some(&top) = stack.last() {
                    res[top] += val - prev;
                }
                prev = val;
                stack.push(id);
            }
            Action::End => {
                if let Some(top) = stack.pop() {
                    res[top] += val + 1 - prev;
                    prev = val + 1;
                }
            }
        }
    }
    res
}

enum Action {
    Start,
    End,
}
// stack
#[test]
fn test1_636() {
    assert_eq!(
        exclusive_time(
            2,
            vec![
                String::from("0:start:0"),
                String::from("1:start:2"),
                String::from("1:end:5"),
                String::from("0:end:6")
            ]
        ),
        vec![3, 4]
    );
}
