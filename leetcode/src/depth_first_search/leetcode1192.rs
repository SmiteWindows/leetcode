// https://leetcode-cn.com/problems/critical-connections-in-a-network/
// Runtime: 168 ms
// Memory Usage: 19.6 MB
pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut graph = vec![Vec::new(); n];
    for connection in connections {
        let u = connection[0] as usize;
        let v = connection[1] as usize;
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut time = 0;
    let mut nodes = (0..n).map(Node::new).collect::<Vec<_>>();
    let mut res = Vec::new();
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
    use leetcode_prelude::vec2;
    assert_eq!(
        critical_connections(4, vec2![[0, 1], [1, 2], [2, 0], [1, 3]]),
        vec2![[1, 3]]
    );
}
