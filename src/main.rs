#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::input;

fn main() {
    input!{
        N: usize,
        M: usize,
        H: [usize; N],
        UV: [[usize; 2]; M]
    }
    let mut graph = vec![Vec::new(); N];
    for uv in UV.iter() {
        let u = uv[0] - 1;let v = uv[1] - 1;
        graph[u].push(v);graph[v].push(u);
    }
    let mut seen = vec![-2*1000_000_000; N];
    let mut deque = VecDeque::new();
    seen[0] = 0;
    deque.push_back(0);
    while deque.len() > 0 {
        let next = deque.pop_front().unwrap();
        for v in graph[next].iter() {
            let cost: i128 = if H[next] >= H[*v] { (H[next] - H[*v]) as i128 } else { 2*(H[next] as i128 - H[*v] as i128)};
            if seen[next] as i128 + cost <= seen[*v] { continue; }
            seen[*v] = seen[next] as i128 + cost;
            deque.push_back(*v);
        }
    }
    println!("{}", seen.iter().max().unwrap());
}