use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::{Usize1, Isize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        H: [usize; N],
        UV: [(Usize1, Usize1); M],
    }
    let mut graph:Vec<Vec<(usize, isize)>> = vec![Vec::new();N];
    for (u,v) in UV.iter() {
        graph[*u].push((*v, if H[*u] < H[*v] { (H[*v] - H[*u] ) as isize} else { 0 }));
        graph[*v].push((*u,if H[*v] < H[*u] { (H[*u] - H[*v]) as isize } else { 0 }));
    }
    let unreach_value = 1000_000_000;
    let mut cost_table = vec![unreach_value; N];
    cost_table[0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push((0isize, 0));
    while heap.len() > 0 {
        let (cost, v) = heap.pop().unwrap();
        for (nv, n_cost) in graph[v].iter() {
            let next_cost = (-cost) + *n_cost;
            if cost_table[*nv] <= next_cost { continue; }
            cost_table[*nv] = next_cost;
            heap.push((-next_cost, *nv));
        }
    }
    let mut ans = 0;
    for i in 0..N{
        let tmp_sum = H[0] as isize - H[i] as isize - cost_table[i];
        ans = max(ans, tmp_sum);
    }
    println!("{}",ans);
}