// https://leetcode-cn.com/problems/minimum-genetic-mutation/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::{HashMap, HashSet, VecDeque};
pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
    let mut hs = HashSet::new();
    let mut mutation_bank = HashSet::new();
    hs.insert(start.clone());
    hs.insert(end.clone());
    for gene in bank {
        hs.insert(gene.clone());
        mutation_bank.insert(gene);
    }
    let n = hs.len();
    let mut hm = HashMap::new();
    for (i, gene) in hs.into_iter().enumerate() {
        hm.insert(gene, i);
    }
    let mut edges = vec![Vec::new(); n];
    for (gene, &u) in &hm {
        let gene = gene.chars().collect::<Vec<_>>();
        let n = gene.len();
        let mut mutation = gene.to_vec();
        for i in 0..n {
            for &c in &['A', 'C', 'G', 'T'] {
                if c != gene[i] {
                    mutation[i] = c;
                    let new_gene = mutation.iter().collect::<String>();
                    if !mutation_bank.contains(&new_gene) {
                        continue;
                    }
                    if let Some(&v) = &hm.get(&new_gene) {
                        edges[u].push(v);
                    }
                }
            }
            mutation[i] = gene[i];
        }
    }
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    let start_id = hm[&start];
    let end_id = hm[&end];
    queue.push_back((start_id, 0));
    while let Some((u, d)) = queue.pop_front() {
        visited[u] = true;
        if u == end_id {
            return d;
        }
        for &v in &edges[u] {
            if !visited[v] {
                queue.push_back((v, d + 1));
            }
        }
    }
    -1
}
#[test]
fn test433() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        min_mutation(
            "AACCGGTT".to_string(),
            "AACCGGTA".to_string(),
            vec_string!["AACCGGTA"]
        ),
        1
    );
    assert_eq!(
        min_mutation(
            "AACCGGTT".to_string(),
            "AAACGGTA".to_string(),
            vec_string!["AACCGGTA", "AACCGCTA", "AAACGGTA"]
        ),
        2
    );
    assert_eq!(
        min_mutation(
            "AAAAACCC".to_string(),
            "AACCCCCC".to_string(),
            vec_string!["AAAACCCC", "AAACCCCC", "AACCCCCC"]
        ),
        3
    );
    assert_eq!(
        min_mutation("AACCGGTT".to_string(), "AACCGGTA".to_string(), vec![]),
        -1
    );
}
