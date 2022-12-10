// https://atcoder.jp/contests/soundhound2018-summer-qual/tasks/soundhound2018_summer_qual_d
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
const FIRST_VALUE: usize = 1000_000_000_000_000;
const MOD: usize = 1_000_000_007;

fn dijkstra(
    x: usize,
    graph: &Vec<Vec<(usize, isize, isize)>>,
    cost_graph: &mut Vec<isize>,
    is_yen: bool,
) {
    let mut pq = BinaryHeap::new();
    cost_graph[x] = 0;
    pq.push((0, x));

    while !pq.is_empty() {
        let (_, nx) = pq.pop().unwrap();
        for (y, yen, snuuk) in graph[nx].iter() {
            let c = if is_yen { *yen } else { *snuuk };
            if cost_graph[*y] <= cost_graph[nx] - c {
                continue;
            }
            cost_graph[*y] = cost_graph[nx] - c;
            pq.push((-cost_graph[*y], *y));
        }
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        S: Usize1,
        T: Usize1,
        UVAB: [(Usize1, Usize1, isize, isize); M]
    }
    let mut yen = vec![FIRST_VALUE as isize; N];
    let mut snuuk = vec![FIRST_VALUE as isize; N];
    let mut graph = vec![vec![]; N];

    for (u, v, a, b) in UVAB.into_iter() {
        graph[u].push((v, -a, -b));
        graph[v].push((u, -a, -b));
    }

    dijkstra(S, &graph, &mut yen, true);
    dijkstra(T, &graph, &mut snuuk, false);

    let mut ans = vec![];
    let mut max_left = 0isize;
    for i in (0..N).rev() {
        let mut left = 10isize.pow(15);
        left -= yen[i];
        left -= snuuk[i];
        max_left = max_left.max(left);
        ans.push(max_left);
    }
    println!("{}", ans.into_iter().rev().join("\n"));
}
