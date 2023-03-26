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
use std::{
    fmt::Debug,
    io::{stdout, Write},
    mem::swap,
};
fn unwrap_result_type<T: Debug>(x: Result<T, T>) -> T {
    if x.is_ok() {
        x.unwrap()
    } else {
        x.unwrap_err()
    }
}

fn read() -> usize {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<usize>().unwrap()
}
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = std::usize::MAX;
const MOD: usize = 1_000_000_007;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
        AB: [(Usize1, Usize1); N-1],
        PX: [(Usize1, usize); Q],
    }
    let mut ans = vec![0; N];
    let mut add_num = vec![0; N];
    let mut graph = vec![vec![]; N];
    let mut passed = vec![false; N];

    for (a, b) in AB.into_iter() {
        graph[a].push(b);
        graph[b].push(a);
    }

    for (p, x) in PX.into_iter() {
        add_num[p] += x;
    }

    let mut heap = VecDeque::new();
    heap.push_back((0, add_num[0]));
    ans[0] = add_num[0];
    passed[0] = true;
    while heap.len() > 0 {
        let (i, n) = heap.pop_front().unwrap();
        for ni in graph[i].iter() {
            if passed[*ni] {
                continue;
            }
            ans[*ni] = n + add_num[*ni];
            passed[*ni] = true;
            heap.push_back((*ni, ans[*ni]));
        }
    }

    println!("{}", ans.into_iter().join(" "));
}
