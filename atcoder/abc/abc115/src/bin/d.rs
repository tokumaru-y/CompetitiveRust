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

fn dfs(N: usize, X: usize, total: &Vec<usize>, patty: &Vec<usize>) -> usize {
    if N == 0 || X <= 0 {
        if X <= 0 {
            0
        } else {
            1
        }
    } else if X <= 1 + total[N - 1] {
        dfs(N - 1, X - 1, total, patty)
    } else if X == 2 + total[N - 1] {
        patty[N - 1] + 1
    } else {
        patty[N - 1] + 1 + dfs(N - 1, X - 2 - total[N - 1], total, patty)
    }
}
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
    }
    let mut total = vec![1];
    let mut patty = vec![1];
    for i in 0..N {
        total.push(total[i] * 2 + 3);
        patty.push(patty[i] * 2 + 1)
    }

    let ans = dfs(N, X, &total, &patty);
    println!("{}", ans);
}
