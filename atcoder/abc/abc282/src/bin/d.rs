#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Isize1, Usize1},
};
use std::fmt::Debug;
#[allow(unused_imports)]
use std::{
    cmp::Reverse,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    process::exit,
};
fn unwrap_result_type<T: Debug>(x: Result<T, T>) -> T {
    if x.is_ok() {
        x.unwrap()
    } else {
        x.unwrap_err()
    }
}
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;

fn bfs(
    dist: &mut Vec<isize>,
    graph: &Vec<Vec<usize>>,
    parent: usize,
    bw_cnt: &mut Vec<(usize, usize)>,
) -> bool {
    let mut vq = VecDeque::new();
    dist[parent] = 0;
    let mut b_cnt = 1;
    let mut w_cnt = 0;

    vq.push_back(parent);
    while vq.len() > 0 {
        let x = vq.pop_front().unwrap();
        for nx in graph[x].iter() {
            if *nx == x {
                continue;
            }
            if dist[*nx] != -1 {
                if dist[*nx] % 2 == dist[x] % 2 {
                    return false;
                } else {
                    continue;
                }
            }
            dist[*nx] = dist[x] + 1;
            if dist[*nx] % 2 == 0 {
                b_cnt += 1;
            } else {
                w_cnt += 1;
            }
            vq.push_back(*nx);
        }
    }
    bw_cnt.push((b_cnt, w_cnt));
    true
}
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        UV: [(Usize1, Usize1); M]
    }
    let mut dist = vec![-1; N];
    let mut graph = vec![vec![]; N];

    for (u, v) in UV.into_iter() {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut is_ok = true;
    let mut black_white_cnt = vec![];
    for i in 0..N {
        if dist[i] == -1 {
            is_ok &= bfs(&mut dist, &graph, i, &mut black_white_cnt);
        }
    }

    if is_ok {
        let mut dif = M;
        for (b, w) in black_white_cnt.into_iter() {
            if b >= 1 {
                dif += b * (b - 1) / 2;
            }
            if w >= 1 {
                dif += w * (w - 1) / 2;
            }
        }
        let ans: usize = (N * (N - 1)) / 2 - dif;
        println!("{}", ans);
    } else {
        println!("0");
    }
}
