// https://atcoder.jp/contests/abc126/tasks/abc126_d
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Isize1, Usize1},
};
#[allow(unused_imports)]
use std::{
    cmp::Reverse,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    process::exit,
};
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;

fn dfs(x: usize, dist: &mut Vec<isize>, colors: &mut Vec<isize>, graph: &Vec<Vec<(usize, usize)>>) {
    for (nx, d) in graph[x].iter() {
        if colors[*nx] != -1 {
            continue;
        }
        dist[*nx] = *d as isize;
        colors[*nx] = if dist[*nx] % 2 == 0 {
            colors[x]
        } else {
            (colors[x] + 1) % 2
        };
        dfs(*nx, dist, colors, graph);
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        UVW: [(Usize1, Usize1, usize); N-1]
    }
    let mut dist = vec![0; N];
    let mut colors = vec![-1; N];
    let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; N];

    for (u, v, w) in UVW.into_iter() {
        graph[u].push((v, w));
        graph[v].push((u, w));
    }

    colors[0] = 0;
    dfs(0, &mut dist, &mut colors, &graph);

    for c in colors.into_iter() {
        println!("{}", c);
    }
}
