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
const FIRST_VALUE: isize = std::isize::MIN;
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        TXA: [(usize, usize, isize); N]
    }
    let ll = 10usize.pow(5);
    let mut dp = vec![vec![FIRST_VALUE; 5]; ll + 1];
    dp[0][0] = 0;
    let mut time_table = vec![vec![]; ll + 1];

    for (t, x, a) in TXA.into_iter() {
        time_table[t].push((x, a));
    }

    for i in 0..(ll) {
        for j in 0..5 {
            if dp[i][j] == FIRST_VALUE {
                continue;
            }
            for (x, a) in time_table[i + 1].iter() {
                if !(*x == j + 1 || *x + 1 == j || *x == j) {
                    continue;
                }
                dp[i + 1][*x] = max(dp[i + 1][*x], dp[i][j] + *a);
            }

            for d in [-1, 0, 1].into_iter() {
                let nd = j as isize + *d;
                if !(0 <= nd && nd <= 4) {
                    continue;
                }
                let nd = nd as usize;
                dp[i + 1][nd] = max(dp[i + 1][nd], dp[i][j]);
            }
        }
    }
    println!("{}", dp[ll].iter().max().unwrap());
}
