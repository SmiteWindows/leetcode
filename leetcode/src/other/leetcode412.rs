// https://leetcode.com/problems/fizz-buzz/
// Runtime: 0 ms
// Memory Usage: 2.7 MB
pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut res = vec![];
    for i in 1..=n {
        let fizz = i % 3 == 0;
        let buzz = i % 5 == 0;
        let s = match (fizz, buzz) {
            (true, true) => "FizzBuzz".to_string(),
            (true, false) => "Fizz".to_string(),
            (false, true) => "Buzz".to_string(),
            (false, false) => format!("{}", i),
        };
        res.push(s);
    }
    res
}
#[test]
fn test412() {
    assert_eq!(
        fizz_buzz(15),
        vec![
            "1".to_string(),
            "2".to_string(),
            "Fizz".to_string(),
            "4".to_string(),
            "Buzz".to_string(),
            "Fizz".to_string(),
            "7".to_string(),
            "8".to_string(),
            "Fizz".to_string(),
            "Buzz".to_string(),
            "11".to_string(),
            "Fizz".to_string(),
            "13".to_string(),
            "14".to_string(),
            "FizzBuzz".to_string()
        ]
    );
}
