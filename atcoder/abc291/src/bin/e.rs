#[allow(unused_imports)]
use itertools::Itertools;
use petgraph::graphmap;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Isize1, Usize1},
};
use rand::seq::index;
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
        M: usize,
        XY: [(Usize1, Usize1); M]
    }
    let mut index_list = vec![N + 1; N];
    let mut set = BTreeSet::new();
    let mut graph = vec![N + 1; N];
    let mut is_edge_cnt = vec![0; N];
    let mut has_edge_cnt = vec![0; N];

    for (x, y) in XY.into_iter() {
        set.insert((x, y));
    }
    for (x, y) in set.into_iter() {
        graph[y] = x;
        is_edge_cnt[x] += 1;
        has_edge_cnt[y] += 1;
    }

    let mut is_ok = true;
    let mut is_zero_cnt = 0;
    let mut start_index = N + 1;
    for i in 0..N {
        if is_edge_cnt[i] == 0 && has_edge_cnt[i] > 0 {
            is_zero_cnt += 1;
            start_index = i;
        }

        if is_edge_cnt[i] >= 2 || is_zero_cnt >= 2 || has_edge_cnt[i] >= 2 {
            is_ok = false;
        }
    }

    if !is_ok {
        println!("No");
        exit(0);
    }

    let mut deque = VecDeque::new();
    let mut num = N;
    deque.push_back(start_index);
    while deque.len() > 0 {
        let nx = deque.pop_back().unwrap();
        index_list[nx] = num;
        num -= 1;
        if graph[nx] == N + 1 {
            break;
        }
        deque.push_back(graph[nx]);
    }
    println!("Yes");
    println!("{}", index_list.into_iter().join(" "));
}
