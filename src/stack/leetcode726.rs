// https://leetcode.com/problems/number-of-atoms/
pub fn count_of_atoms(formula: String) -> String {
    todo!()
}
// hash_table stack recursion
#[test]
#[ignore]
fn test3_726() {
    assert_eq!(count_of_atoms(String::from("H2O")), String::from("H2O"));
    assert_eq!(
        count_of_atoms(String::from("Mg(OH)2")),
        String::from("H2MgO2")
    );
    assert_eq!(
        count_of_atoms(String::from("K4(ON(SO3)2)2")),
        String::from("K4N2O14S4")
    );
}
