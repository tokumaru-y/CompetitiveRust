// https://atcoder.jp/contests/joi2008yo/tasks/joi2008yo_f
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
fn dijkstra(x: usize, graph: &Vec<Vec<(usize, isize)>>, cost_graph: &mut Vec<isize>) {
    let mut pq = BinaryHeap::new();
    cost_graph[x] = 0;
    pq.push((0, x));

    while !pq.is_empty() {
        let (_, nx) = pq.pop().unwrap();
        for (y, cost) in graph[nx].iter() {
            let c = *cost;
            if cost_graph[*y] <= cost_graph[nx] + c {
                continue;
            }
            cost_graph[*y] = cost_graph[nx] + c;
            pq.push((-cost_graph[*y], *y));
        }
    }
}

const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut graph = vec![vec![]; N];
    for i in 0..K {
        input! {
            T: usize
        }
        if T == 0 {
            input! {
                x: Usize1,
                y: Usize1,
            }
            let mut cost_graph = vec![FIRST_VALUE as isize; N];
            dijkstra(x, &graph, &mut cost_graph);

            let ans = if cost_graph[y] == FIRST_VALUE as isize {
                -1
            } else {
                cost_graph[y]
            };
            println!("{}", ans);
        } else {
            input! {
                x: Usize1,
                y: Usize1,
                w: isize,
            }
            graph[x].push((y, w));
            graph[y].push((x, w));
        }
    }
}
