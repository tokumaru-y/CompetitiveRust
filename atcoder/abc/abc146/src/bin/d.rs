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
use std::{
    fmt::Debug,
    io::{stdout, Write},
    mem::swap,
};
fn unwrap_result_type<T: Debug>(x: Result<T, T>) -> T {
    if x.is_ok() {
        x.unwrap()
    } else {
        x.unwrap_err()
    }
}

fn read() -> usize {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<usize>().unwrap()
}
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = std::usize::MAX;
const MOD: usize = 1_000_000_007;

fn dfs(graph: &Vec<Vec<(usize, usize)>>, colors: &mut Vec<usize>, x: usize, pre_color: usize) {
    let mut next = vec![];
    for _x in graph[x].iter() {
        let (nx, node) = _x;
        if colors[*node] > 0 {
            continue;
        }
        next.push((*nx, *node));
    }
    let mut next_color = 1;
    for (nx, node) in next.into_iter() {
        if next_color == pre_color {
            next_color += 1;
        }
        colors[node] = next_color;
        dfs(graph, colors, nx, next_color);
        next_color += 1;
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        AB: [(Usize1, Usize1); N-1]
    }
    let mut colors = vec![0; N - 1];
    let mut graph = vec![vec![]; N];
    for i in 0..(N - 1) {
        let (a, b) = AB[i];
        graph[a].push((b, i as usize));
        graph[b].push((a, i as usize));
    }
    dfs(&graph, &mut colors, 0, 0);
    println!("{}", colors.iter().max().unwrap());
    for a in colors.into_iter() {
        println!("{}", a);
    }
}
