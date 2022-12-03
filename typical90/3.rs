// https://atcoder.jp/contests/typical90/tasks/typical90_c
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
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
    process::exit,
};
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;
fn dfs(ind: usize, graph: &Vec<Vec<usize>>, dist: &mut Vec<isize>) {
    for i in graph[ind].iter() {
        if dist[*i] != -1 {
            continue;
        }
        dist[*i] = dist[ind] + 1;
        dfs(*i, graph, dist);
    }
}
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        AB: [(Usize1, Usize1); N - 1]
    }
    let mut graph = vec![vec![]; N];
    for (a, b) in AB.into_iter() {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut dist = vec![-1; N];
    dist[0] = 0;
    dfs(0, &graph, &mut dist);
    let max_dist = dist.iter().max().unwrap();
    let mut s_ind = 0;
    for i in 0..N {
        if dist[i] == *max_dist {
            s_ind = i;
            break;
        }
    }

    let mut dist = vec![-1; N];
    dist[s_ind] = 0;
    dfs(s_ind, &graph, &mut dist);
    println!("{}", dist.iter().max().unwrap() + 1);
}
