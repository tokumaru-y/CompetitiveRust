use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
use std::ops::Bound::Included;
use petgraph::graph;
use proconio::{input, marker::Chars};
#[derive(Clone)]
struct Edge {
    to: usize,
    cost: usize,
}

fn dfs(ans: &mut Vec<usize>, graph: &Vec<Vec<Edge>>, n: usize, color: usize) {
    ans[n] = color;
    for e in graph[n].iter() {
        if e.to == n { continue; }
        if ans[e.to] != 2 { continue; }
        let next_color = if (e.cost) % 2 == 0 { color } else { (color + 1) % 2};
        dfs(ans, graph, e.to, next_color);
    } 
}
fn main() {
    input! {
        N: usize,
        UVW: [[usize; 3]; N-1],
    }
    let mut graph = vec![Vec::new(); N];
    for uvw in UVW.iter() {
        let u = uvw[0] - 1;let v = uvw[1] - 1;let w = uvw[2];
        graph[u].push(Edge{ to: v, cost: w});
        graph[v].push(Edge{ to: u, cost: w});
    }
    let mut ans = vec![2; N];
    dfs(&mut ans, &graph,0, 0);
    for a in ans.iter() {
        println!("{}", a);
    }
}