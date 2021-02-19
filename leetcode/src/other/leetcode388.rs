// https://leetcode-cn.com/problems/longest-absolute-file-path/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn length_longest_path(input: String) -> i32 {
    let mut stack: Vec<Folder> = Vec::new();
    let mut folder_length = 0;
    let mut max = 0;
    for line in input.split('\n') {
        let s = line.chars();
        let mut level = 0;
        let mut is_folder = true;
        for c in s {
            match c {
                '\t' => {
                    level += 1;
                }
                '.' => {
                    is_folder = false;
                    break;
                }
                _ => {}
            }
        }
        let name = line[level..].to_string();
        while let Some(top) = stack.pop() {
            if top.level >= level {
                folder_length -= top.length;
            } else {
                stack.push(top);
                break;
            }
        }
        if is_folder {
            let length = name.len() + 1;
            let folder = Folder { level, length };
            stack.push(folder);
            folder_length += length;
        } else {
            max = max.max(name.len() + folder_length);
        }
    }
    max as i32
}

struct Folder {
    level: usize,
    length: usize,
}
#[test]
fn test388() {
    assert_eq!(length_longest_path("dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
            .to_string()), 32);
}
