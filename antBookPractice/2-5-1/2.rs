// https://atcoder.jp/contests/code-festival-2017-qualb/tasks/code_festival_2017_qualb_c
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

fn dfs(x: usize, graph: &Vec<Vec<usize>>, colors: &mut Vec<isize>) -> bool {
    let mut res = true;
    for nx in graph[x].iter() {
        if x == *nx {
            continue;
        }
        if colors[*nx] != -1 && colors[*nx] == colors[x] {
            return false;
        }
        if colors[*nx] != -1 {
            continue;
        }
        colors[*nx] = (colors[x] + 1) % 2;
        res &= dfs(*nx, graph, colors);
    }
    res
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1); M]
    }
    let mut graph = vec![vec![]; N];
    let mut colors = vec![-1; N];

    for (a, b) in AB.into_iter() {
        graph[a].push(b);
        graph[b].push(a);
    }

    colors[0] = 0;
    let is_b_graph = dfs(0, &graph, &mut colors);

    let ans = if is_b_graph {
        let black = colors.iter().filter(|x| **x == 1).count();
        let white = colors.iter().filter(|x| **x == 0).count();
        black * white - M
    } else {
        N * (N - 1) / 2 - M
    };
    println!("{}", ans);
}
