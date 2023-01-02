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
const FIRST_VALUE: usize = 1_000_000_000_000_000;
const MOD: usize = 1_000_000_007;
fn bfs(
    dist: &mut Vec<Vec<usize>>,
    graph: &Vec<Vec<Vec<usize>>>,
    status: usize,
    top: usize,
    switch_node: &mut Vec<bool>,
) {
    let mut deque = VecDeque::new();
    deque.push_back((top, status));

    while deque.len() > 0 {
        let (x, nstatus) = deque.pop_front().unwrap();

        for nx in graph[nstatus][x].iter() {
            if dist[nstatus][*nx] != FIRST_VALUE {
                continue;
            }

            dist[nstatus][*nx] = dist[nstatus][x] + 1;
            if *nx == switch_node.len() - 1 {
                continue;
            }
            deque.push_back((*nx, nstatus));
        }

        if switch_node[x] == true {
            for nx in graph[(nstatus + 1) % 2][x].iter() {
                if dist[(nstatus + 1) % 2][*nx] != FIRST_VALUE {
                    continue;
                }

                dist[(nstatus + 1) % 2][*nx] = dist[nstatus][x] + 1;
                if *nx == switch_node.len() - 1 {
                    continue;
                }
                deque.push_back((*nx, (nstatus + 1) % 2));
            }
        }
    }
}
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
        UVA: [(Usize1, Usize1, usize); M],
        S: [Usize1; K]
    }
    let mut graph = vec![vec![vec![]; N]; 2];
    let mut can_move_edge_status = 1;
    let mut dist = vec![vec![FIRST_VALUE; N]; 2];
    let mut switch_node = vec![false; N];

    dist[can_move_edge_status][0] = 0;
    for s in S.into_iter() {
        switch_node[s] = true;
    }
    for (u, v, a) in UVA.into_iter() {
        graph[a][u].push(v);
        graph[a][v].push(u);
    }

    bfs(&mut dist, &graph, can_move_edge_status, 0, &mut switch_node);
    let mut ans = min(dist[0][N - 1], dist[1][N - 1]);
    if ans == FIRST_VALUE {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}
