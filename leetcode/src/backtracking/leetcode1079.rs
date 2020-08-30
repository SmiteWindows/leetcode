// https://leetcode-cn.com/problems/letter-tile-possibilities/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn num_tile_possibilities(tiles: String) -> i32 {
    let mut counts: Vec<usize> = vec![0; 26];
    for c in tiles.chars() {
        counts[(c as u8 - b'A') as usize] += 1;
    }
    let mut res = 0;
    dfs(&mut res, &mut counts);
    res
}

fn dfs(res: &mut i32, counts: &mut Vec<usize>) {
    for i in 0..26 {
        if counts[i] > 0 {
            *res += 1;
            counts[i] -= 1;
            dfs(res, counts);
            counts[i] += 1;
        }
    }
}
// backtracking
#[test]
fn test1_1079() {
    assert_eq!(num_tile_possibilities(String::from("AAB")), 8);
    assert_eq!(num_tile_possibilities(String::from("AAABBC")), 188);
}
