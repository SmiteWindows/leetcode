// https://leetcode.com/problems/groups-of-special-equivalent-strings/
pub fn num_special_equiv_groups(a: Vec<String>) -> i32 {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_893() {
    assert_eq!(
        num_special_equiv_groups(vec![
            String::from("abcd"),
            String::from("cdab"),
            String::from("cbad"),
            String::from("xyzz"),
            String::from("zzxy"),
            String::from("zzyx")
        ]),
        3
    );
}
