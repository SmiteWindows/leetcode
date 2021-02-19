// https://leetcode-cn.com/problems/number-of-atoms/
pub fn count_of_atoms(formula: String) -> String {
    todo!()
}
// hash_table stack recursion
#[test]
#[ignore]
fn test3_726() {
    assert_eq!(count_of_atoms("H2O".to_string()), "H2O".to_string());
    assert_eq!(count_of_atoms("Mg(OH)2".to_string()), "H2MgO2".to_string());
    assert_eq!(
        count_of_atoms("K4(ON(SO3)2)2".to_string()),
        "K4N2O14S4".to_string()
    );
}
