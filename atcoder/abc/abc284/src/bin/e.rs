#[allow(unused_imports)]
use itertools::Itertools;
use num_traits::Pow;
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
const FIRST_VALUE: isize = std::isize::MIN;
const MOD: usize = 1_000_000_007;

fn dfs(graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, ans: &mut usize, x: usize) {
    for nx in graph[x].iter() {
        if seen[*nx] {
            continue;
        }
        *ans += 1;
        if *ans >= 10usize.pow(6) {
            return;
        }
        seen[*nx] = true;
        dfs(graph, seen, ans, *nx);
        seen[*nx] = false;
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        UV: [(Usize1, Usize1); M]
    }
    let mut seen = vec![false; N];
    let mut graph = vec![vec![]; N];

    for (u, v) in UV.into_iter() {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut ans = 1;
    seen[0] = true;
    for i in graph[0].iter() {
        seen[*i] = true;
        ans += 1;
        if ans >= 10usize.pow(6) {
            break;
        }
        dfs(&graph, &mut seen, &mut ans, *i);
        seen[*i] = false;
    }

    println!("{}", min(ans, 10usize.pow(6)));
}
