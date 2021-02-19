// https://leetcode-cn.com/problems/get-watched-videos-by-your-friends/
// Runtime: 28 ms
// Memory Usage: 3.1 MB
use std::collections::{HashMap, VecDeque};
pub fn watched_videos_by_friends(
    watched_videos: Vec<Vec<String>>,
    friends: Vec<Vec<i32>>,
    id: i32,
    level: i32,
) -> Vec<String> {
    let n = watched_videos.len();
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    let id = id as usize;
    visited[id] = true;
    queue.push_back((id, 0));
    let mut freq: HashMap<String, usize> = HashMap::new();
    while let Some((u, l)) = queue.pop_front() {
        if l < level {
            for &friend in &friends[u] {
                let v = friend as usize;
                if !visited[v] {
                    visited[v] = true;
                    queue.push_back((v, l + 1));
                }
            }
        } else {
            for video in &watched_videos[u] {
                *freq.entry(video.to_string()).or_default() += 1;
            }
        }
    }
    let mut pairs: Vec<(usize, String)> = Vec::new();
    for (video, count) in freq {
        pairs.push((count, video));
    }
    pairs.sort_unstable();
    pairs.into_iter().map(|p| p.1).collect()
}
// hash_table string breadth_first_search
#[test]
fn test3_1311() {
    use leetcode_prelude::{vec2, vec2_string, vec_string};
    assert_eq!(
        watched_videos_by_friends(
            vec2_string![["A", "B"], ["C"], ["B", "C"], ["D"]],
            vec2![[1, 2], [0, 3], [0, 3], [1, 2]],
            0,
            1
        ),
        vec_string!["B", "C"]
    );
    assert_eq!(
        watched_videos_by_friends(
            vec2_string![["A", "B"], ["C"], ["B", "C"], ["D"]],
            vec2![[1, 2], [0, 3], [0, 3], [1, 2]],
            0,
            2
        ),
        vec_string!["D"]
    );
}
