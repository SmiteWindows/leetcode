// https://leetcode.com/problems/baseball-game/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn cal_points(ops: Vec<String>) -> i32 {
    let mut stack = vec![];
    for s in ops {
        match s.as_ref() {
            "C" => {
                stack.pop();
            }
            "D" => {
                let top = stack.pop().expect("exist");
                let double = top * 2;
                stack.push(top);
                stack.push(double);
            }
            "+" => {
                let b = stack.pop().expect("exist");
                let a = stack.pop().expect("exist");
                let plus = a + b;
                stack.push(a);
                stack.push(b);
                stack.push(plus);
            }
            _ => {
                stack.push(s.parse::<i32>().expect("exist"));
            }
        }
    }
    stack.iter().sum()
}
// stack
#[test]
fn test1_682() {
    assert_eq!(
        cal_points(vec![
            String::from("5"),
            String::from("2"),
            String::from("C"),
            String::from("D"),
            String::from("+")
        ]),
        30
    );
    assert_eq!(
        cal_points(vec![
            String::from("5"),
            String::from("-2"),
            String::from("4"),
            String::from("C"),
            String::from("D"),
            String::from("9"),
            String::from("+"),
            String::from("+")
        ]),
        27
    );
}
