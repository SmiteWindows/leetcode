// https://leetcode-cn.com/problems/bulls-and-cows/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn get_hint(secret: String, guess: String) -> String {
    let s = secret.chars().collect::<Vec<_>>();
    let g = guess.chars().collect::<Vec<_>>();
    let mut bulls = 0;
    let mut cows = 0;
    let mut s_count = vec![0; 10];
    let mut g_count = vec![0; 10];
    let n = s.len();
    for i in 0..n {
        if s[i] == g[i] {
            bulls += 1;
        } else {
            s_count[(s[i] as u8 - b'0') as usize] += 1;
            g_count[(g[i] as u8 - b'0') as usize] += 1;
        }
    }
    for i in 0..10 {
        cows += s_count[i].min(g_count[i]);
    }
    format!("{}A{}B", bulls, cows)
}
// hash_table
#[test]
fn test1_299() {
    assert_eq!(
        get_hint("1807".to_string(), "7810".to_string()),
        "1A3B".to_string()
    );
    assert_eq!(
        get_hint("1123".to_string(), "0111".to_string()),
        "1A1B".to_string()
    );
}
