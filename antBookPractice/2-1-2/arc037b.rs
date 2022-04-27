use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse};
use std::ops::Bound::Included;
use petgraph::graph;
use proconio::{input, marker::Chars};

fn dfs(graph: &Vec<Vec<usize>>, check_list: &mut Vec<bool>, i: usize, parent: usize) -> bool{
    check_list[i] = true;
    let mut res = true;
    for n in graph[i].iter() {
        if parent == *n { continue; }
        if check_list[*n] { return false; }
        res &= dfs(graph, check_list, *n, i);
    }
    res
}

fn main() {
    input! {
        N: usize,
        M: usize,
        UV: [[usize; 2]; M],
    }
    let mut graph = vec![Vec::new(); N];
    for uv in UV.iter() {
        let u = uv[0] - 1;let v = uv[1] - 1;
        graph[u].push(v);graph[v].push(u);
    }
    let mut check_list = vec![false; N];
    let mut ans = 0;
    for i in 0..N{
        if !check_list[i] {
            if dfs(&graph, &mut check_list, i, 20000){
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}