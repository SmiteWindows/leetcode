// https://leetcode-cn.com/problems/video-stitching/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::cmp::Reverse;
pub fn video_stitching(clips: Vec<Vec<i32>>, t: i32) -> i32 {
    let mut clips = clips;
    clips.sort_by_key(|v| (v[0], Reverse(v[1])));
    let n = clips.len();
    let mut res = 0;
    let mut left = 0;
    let mut right = 0;
    for clip in clips.iter() {
        if clip[0] >= t {
            break;
        }
        if clip[0] < left {
            right = right.max(clip[1]);
        } else if clip[0] <= right {
            right = right.max(clip[1]);
            left = right;
            res += 1;
        } else {
            return -1;
        }
    }
    if right < t {
        -1
    } else if left < t {
        res + 1
    } else {
        res
    }
}
// dynamic_programming
#[test]
fn test1_1024() {
    use leetcode_prelude::vec2;
    assert_eq!(
        video_stitching(vec2![[0, 2], [4, 6], [8, 10], [1, 9], [1, 5], [5, 9]], 10),
        3
    );
    assert_eq!(video_stitching(vec2![[0, 1], [1, 2]], 5), -1);
    assert_eq!(
        video_stitching(
            vec2![
                [0, 1],
                [6, 8],
                [0, 2],
                [5, 6],
                [0, 4],
                [0, 3],
                [6, 7],
                [1, 3],
                [4, 7],
                [1, 4],
                [2, 5],
                [2, 6],
                [3, 4],
                [4, 5],
                [5, 7],
                [6, 9]
            ],
            9
        ),
        3
    );
    assert_eq!(video_stitching(vec2![[0, 4], [2, 8]], 5), 2);
}
