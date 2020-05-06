// https://leetcode.com/problems/get-watched-videos-by-your-friends/
pub fn watched_videos_by_friends(
    watched_videos: Vec<Vec<String>>,
    friends: Vec<Vec<i32>>,
    id: i32,
    level: i32,
) -> Vec<String> {
    todo!()
}
// hash_table string breadth_first_search
#[test]
#[ignore]
fn test1_1311() {
    assert_eq!(
        watched_videos_by_friends(
            vec![
                vec![String::from("A"), String::from("B")],
                vec![String::from("C")],
                vec![String::from("B"), String::from("C")],
                vec![String::from("D")]
            ],
            vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]],
            0,
            1
        ),
        vec![String::from("B"), String::from("C")]
    );
    assert_eq!(
        watched_videos_by_friends(
            vec![
                vec![String::from("A"), String::from("B")],
                vec![String::from("C")],
                vec![String::from("B"), String::from("C")],
                vec![String::from("D")]
            ],
            vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]],
            0,
            2
        ),
        vec![String::from("D")]
    );
}
