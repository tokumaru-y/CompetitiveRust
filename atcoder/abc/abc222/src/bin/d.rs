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
const MOD: usize = 998244353;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        B: [usize; N]
    }
    let m = 3000;
    let mut dp = vec![vec![0; m + 1]; N + 10];
    dp[0][0] = 1;

    for i in 0..=N {
        for j in 0..m {
            dp[i][j + 1] += dp[i][j];
            dp[i][j + 1] %= MOD;
        }
        if i == N {
            continue;
        }
        for j in A[i]..=B[i] {
            dp[i + 1][j] += dp[i][j];
            dp[i + 1][j] %= MOD;
        }
    }

    println!("{}", dp[N][m] % MOD);
}
