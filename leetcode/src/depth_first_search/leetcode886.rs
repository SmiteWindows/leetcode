// https://leetcode-cn.com/problems/possible-bipartition/
// Runtime: 28 ms
// Memory Usage: 3 MB
pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
    let n = n as usize;
    let mut graph = vec![Vec::new(); n];
    for dislike in dislikes {
        let u = dislike[0] as usize - 1;
        let v = dislike[1] as usize - 1;
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut colors = vec![0; n];
    for i in 0..n {
        if colors[i] == 0 && !dfs(i, 1, &mut colors, &graph, n) {
            return false;
        }
    }
    true
}

fn dfs(u: usize, color: i32, colors: &mut [i32], graph: &[Vec<usize>], n: usize) -> bool {
    colors[u] = color;
    for &v in &graph[u] {
        if colors[v] == color {
            return false;
        }
        if colors[v] == 0 && !dfs(v, -color, colors, graph, n) {
            return false;
        }
    }
    true
}
// depth_first_search
#[test]
fn test1_886() {
    use leetcode_prelude::vec2;
    assert_eq!(possible_bipartition(4, vec2![[1, 2], [1, 3], [2, 4]]), true);
    assert_eq!(
        possible_bipartition(3, vec2![[1, 2], [1, 3], [2, 3]]),
        false
    );
    assert_eq!(
        possible_bipartition(5, vec2![[1, 2], [2, 3], [3, 4], [4, 5], [1, 5]]),
        false
    );
}
