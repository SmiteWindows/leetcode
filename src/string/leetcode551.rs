// https://leetcode.com/problems/student-attendance-record-i/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn check_record(s: String) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    if s.iter().filter(|&c| c == &'A').count() > 1 {
        return false;
    }
    if s.windows(3).filter(|&w| w == ['L', 'L', 'L']).count() > 0 {
        return false;
    }
    true
}
// string
#[test]
fn test1_551() {
    assert_eq!(check_record(String::from("PPALLP")), true);
    assert_eq!(check_record(String::from("PPALLL")), false);
}
