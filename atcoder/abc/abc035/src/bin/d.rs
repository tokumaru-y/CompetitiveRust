use std::{collections::BTreeSet, vec, time};
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::{Usize1, Isize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        T: usize,
        A: [usize; N],
        ABC: [(Usize1, Usize1, usize); M],
    }
    let mut graph = vec![Vec::new(); N];
    let mut rev_graph = vec![Vec::new(); N];
    for (a,b,c) in ABC.iter() {
        graph[*a].push((*b,*c));rev_graph[*b].push((*a, *c));
    }
    let mut time_cost = vec![1000_000_000_000_000isize; N];
    let mut rev_cost = vec![1000_000_000_000_000isize; N];

    time_cost[0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push((0, 0));
    while heap.len() > 0 {
        let (cost, v) = heap.pop().unwrap();
        let cost = -cost;
        for (nv, nc) in graph[v].iter() {
            let next_cost = *nc as isize + cost;
            if next_cost >= time_cost[*nv] { continue; }
            time_cost[*nv] = next_cost;
            heap.push((-next_cost, *nv))
        }
    }

    rev_cost[0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push((0, 0));
    while heap.len() > 0 {
        let (cost, v) = heap.pop().unwrap();
        let cost = -cost;
        for (nv, nc) in rev_graph[v].iter() {
            let next_cost = *nc as isize + cost;
            if next_cost >= rev_cost[*nv] { continue; }
            rev_cost[*nv] = next_cost;
            heap.push((-next_cost, *nv))
        }
    }

    let mut ans = 0;
    for i in 0..N {
        if time_cost[i] == 1000_000_000_000_000isize || rev_cost[i] == 1000_000_000_000_000isize { continue; }
        let left_time = T as isize - (time_cost[i] + rev_cost[i]);
        ans = max(ans, left_time * A[i] as isize);
    }
    println!("{}",ans);
}