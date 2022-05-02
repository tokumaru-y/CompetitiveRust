use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::{Usize1, Isize1};

fn do_dijkstra(cost_list: &mut Vec<isize>, start: usize, graph: &Vec<Vec<(usize, isize)>>) {
    cost_list[start] = 0;
    let mut heap = BinaryHeap::new();
    heap.push((0, start));
    while heap.len() > 0 {
        let (cost, nv) = heap.pop().unwrap();
        let cost = -cost;
        for (v, nc) in graph[nv].iter() {
            let next_cost = *nc + cost;
            if cost_list[*v] <= next_cost { continue; }
            cost_list[*v] = next_cost;
            heap.push((-next_cost, *v));
        }
    }
}

fn main() {
    input! {
        N: usize,
        mut K: usize,
    }
    let mut graph = vec![Vec::new(); N];
    while K > 0 {
        input! {
            command: usize,
        }
        if command == 0 {
            let mut cost_list = vec![1000_000_000_000_000; N];
            input! {
                s: Usize1,
                t: Usize1,
            }
            do_dijkstra(&mut cost_list, s, &graph);
            let ans = if cost_list[t] == 1000_000_000_000_000 { -1 } else { cost_list[t] };
            println!("{}",ans);
        } else {
            input! {
                s: Usize1,
                t: Usize1,
                c: isize,
            }
            graph[s].push((t,c));graph[t].push((s,c));
        }
        K-=1;
    }
}