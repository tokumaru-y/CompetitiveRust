use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::{Usize1, Isize1};

fn do_dijkstra(cost_list: &mut Vec<(isize)>, graph: &Vec<Vec<(usize, isize, isize)>>, start: usize, is_yen: bool) {
    cost_list[start] = 0;
    let mut heap = BinaryHeap::new();
    heap.push((0isize, start));
    while heap.len() > 0 {
        let (cost, v) = heap.pop().unwrap();
        let cost = -cost;
        for nv in graph[v].iter() {
            let next_cost = if is_yen { nv.1 + cost } else { nv.2 + cost };
            if next_cost >= cost_list[nv.0] { continue; }
            cost_list[nv.0] = next_cost;
            heap.push((-next_cost, nv.0))
        }
    }
}

fn main() {
    input! {
        N: usize,
        M: usize,
        S: Usize1,
        T: Usize1,
    }
    let mut graph = vec![Vec::new(); N];
    let mut A = Vec::new();let mut B = Vec::new();
    for _ in 0..M {
        input! {
            u: Usize1,
            v: Usize1, 
            a: isize,
            b: isize,
        }
        graph[u].push((v,a,b));graph[v].push((u,a,b));
        A.push(a);B.push(b);
    }
    let mut ans = Vec::new();
    let mut ja_yen = vec![1000_000_000_000_001; N];
    let mut snuuk = vec![1000_000_000_000_001; N];
    do_dijkstra(&mut ja_yen, &graph, S, true);
    do_dijkstra(&mut snuuk, &graph, T, false);
    let mut min_cost = 1000_000_000_000_000;
    for i in (0..N).rev() {
        let used_amount = ja_yen[i] + snuuk[i];
        min_cost = min(min_cost, used_amount);
        ans.push(1000_000_000_000_000 - min_cost);
    }
    for a in ans.iter().rev() {
        println!("{}",a);
    }
}