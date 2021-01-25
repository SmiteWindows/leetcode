// https://leetcode-cn.com/problems/baseball-game/
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
                let top = stack.pop().unwrap();
                let double = top * 2;
                stack.push(top);
                stack.push(double);
            }
            "+" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let plus = a + b;
                stack.push(a);
                stack.push(b);
                stack.push(plus);
            }
            _ => {
                stack.push(s.parse::<i32>().unwrap());
            }
        }
    }
    stack.iter().sum()
}
// stack
#[test]
fn test1_682() {
    use leetcode_prelude::vec_string;
    assert_eq!(cal_points(vec_string!["5", "2", "C", "D", "+"]), 30);
    assert_eq!(
        cal_points(vec_string!["5", "-2", "4", "C", "D", "9", "+", "+"]),
        27
    );
}
