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

const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = std::usize::MAX;
const MOD: usize = 100000;

fn calc(n: usize) -> usize {
    let mut res = n;
    let mut c = n;
    while c > 0 {
        res += c % 10;
        c /= 10;
    }
    res % MOD
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut K: usize,
    }
    let limit = 10usize.pow(5);
    let mut dp = vec![vec![0; limit + 1]; 61];
    for i in 0..=limit {
        dp[0][i] = calc(i);
    }

    for i in 1..=60 {
        for j in 0..=limit {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    let mut ans = N;
    let mut idx = 0;
    while K > 0 {
        if K & 1 > 0 {
            ans = dp[idx][ans];
        }

        K >>= 1;
        idx += 1;
    }

    println!("{}", ans);
}
