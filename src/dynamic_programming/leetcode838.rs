// https://leetcode.com/problems/push-dominoes/
pub fn push_dominoes(dominoes: String) -> String {
    todo!()
}
// two_pointers dynamic_programming
#[test]
#[ignore]
fn test2_838() {
    assert_eq!(
        push_dominoes(String::from(".L.R...LR..L..")),
        String::from("LL.RR.LLRRLL..")
    );
    assert_eq!(push_dominoes(String::from("RR.L")), String::from("RR.L"));
}
