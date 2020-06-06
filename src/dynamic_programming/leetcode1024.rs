// https://leetcode.com/problems/video-stitching/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn video_stitching(clips: Vec<Vec<i32>>, t: i32) -> i32 {
    use std::cmp::Reverse;
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
    assert_eq!(
        video_stitching(
            vec![
                vec![0, 2],
                vec![4, 6],
                vec![8, 10],
                vec![1, 9],
                vec![1, 5],
                vec![5, 9]
            ],
            10
        ),
        3
    );
    assert_eq!(video_stitching(vec![vec![0, 1], vec![1, 2]], 5), -1);
    assert_eq!(
        video_stitching(
            vec![
                vec![0, 1],
                vec![6, 8],
                vec![0, 2],
                vec![5, 6],
                vec![0, 4],
                vec![0, 3],
                vec![6, 7],
                vec![1, 3],
                vec![4, 7],
                vec![1, 4],
                vec![2, 5],
                vec![2, 6],
                vec![3, 4],
                vec![4, 5],
                vec![5, 7],
                vec![6, 9]
            ],
            9
        ),
        3
    );
    assert_eq!(video_stitching(vec![vec![0, 4], vec![2, 8]], 5), 2);
}
