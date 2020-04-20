// https://leetcode.com/problems/minimum-number-of-frogs-croaking/
pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
   todo!()     
}
// string
#[test]
#[ignore]
fn test1_1419() {
    assert_eq!(min_number_of_frogs(String::from("croakcroak")),1);
    assert_eq!(min_number_of_frogs(String::from("crcoakroak")),2);
    assert_eq!(min_number_of_frogs(String::from("croakcrook")),-1);
    assert_eq!(min_number_of_frogs(String::from("croakcroa")),-1);
}