// https://leetcode.com/problems/critical-connections-in-a-network/
// Runtime: 168 ms
// Memory Usage: 19.6 MB
pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut graph = vec![vec![]; n];
    for connection in connections {
        let u = connection[0] as usize;
        let v = connection[1] as usize;
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut time = 0;
    let mut nodes = (0..n).map(Node::new).collect::<Vec<_>>();
    let mut res = vec![];
    dfs(0, 0, &mut time, &mut nodes, &mut res, &graph);
    res
}

fn dfs(
    u: usize,
    parent: usize,
    time: &mut usize,
    nodes: &mut Vec<Node>,
    edges: &mut Vec<Vec<i32>>,
    graph: &[Vec<usize>],
) {
    nodes[u].discovery = *time;
    nodes[u].lowest = *time;
    *time += 1;
    for &v in &graph[u] {
        if v == parent {
            continue;
        }
        if nodes[v].discovery == usize::MAX {
            dfs(v, u, time, nodes, edges, graph);
            nodes[u].lowest = nodes[u].lowest.min(nodes[v].lowest);
            if nodes[v].lowest > nodes[u].discovery {
                edges.push(vec![u as i32, v as i32]);
            }
        } else {
            nodes[u].lowest = nodes[u].lowest.min(nodes[v].discovery);
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Node {
    id: usize,
    discovery: usize,
    lowest: usize,
    on_stack: bool,
}

impl Node {
    fn new(id: usize) -> Self {
        let discovery = usize::MAX;
        let lowest = usize::MAX;
        let on_stack = false;
        Self {
            id,
            discovery,
            lowest,
            on_stack,
        }
    }
}
// depth_first_search
#[test]
fn test1_1192() {
    assert_eq!(
        critical_connections(4, vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]]),
        vec![vec![1, 3]]
    );
}
