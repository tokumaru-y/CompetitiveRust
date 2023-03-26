#[allow(unused_imports)]
use std::{collections::{BinaryHeap, HashMap, VecDeque, BTreeMap, BTreeSet}, cmp::Reverse, cmp::{max, min}};
#[allow(unused_imports)]
use proconio::{input, marker::{Chars,Usize1, Isize1}};
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, X: usize,
        A: [Usize1; N]
    }
    let mut graph = vec![0; N];
    for i in 0..N {
        graph[i] = A[i];
    }
    let mut passed = vec![false; N];
    let mut deque = VecDeque::new();
    deque.push_back(X-1);
    let mut ans = 0;
    while deque.len() > 0 {
        let n = deque.pop_front().unwrap();
        if passed[n] { continue; }
        passed[n] = true;
        deque.push_back(graph[n]);
        ans += 1;
    }
    println!("{}",ans);
}