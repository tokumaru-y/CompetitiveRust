// https://atcoder.jp/contests/wupc2012-closed/tasks/wupc2012_5
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
const FIRST_VALUE: usize = 1_000_000_000_000_000;
const MOD: usize = 1_000_000_007;
fn dijkstra(
    x: usize,
    graph: &Vec<Vec<(usize, isize)>>,
    cost_graph: &mut Vec<Vec<isize>>,
    last: usize,
) {
    let mut pq = BinaryHeap::new();
    cost_graph[x][0] = 0;
    pq.push((0isize, (x, 0)));

    while !pq.is_empty() {
        let (nc, (nx, ny)) = pq.pop().unwrap();
        for (y, cost) in graph[nx].iter() {
            let c = *cost;
            let n_ind = ((-nc + c) % 28) as usize;
            if cost_graph[*y][n_ind] <= -nc + c && cost_graph[*y][n_ind] >= 0 {
                continue;
            }
            cost_graph[*y][n_ind] = -nc + c;
            if *y == last {
                continue;
            }
            pq.push((-cost_graph[*y][n_ind], (*y, n_ind)));
        }
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        FTC: [(usize, usize, isize); M]
    }
    let mut graph = vec![vec![]; N];

    for (x, y, c) in FTC.into_iter() {
        graph[x].push((y, c));
        graph[y].push((x, c));
    }

    let mut cost_graph = vec![vec![-1; 28]; N];
    dijkstra(0, &graph, &mut cost_graph, N - 1);

    let mut ans = FIRST_VALUE as isize;
    for i in 0..28 {
        if (i % 4 == 0 || i % 7 == 0) && (cost_graph[N - 1][i] != -1) {
            ans = ans.min(cost_graph[N - 1][i]);
        }
    }

    println!("{}", ans);
}
