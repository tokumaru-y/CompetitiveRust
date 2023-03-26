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
const MOD: usize = 1_000_000_007;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut K: usize,
        A: [Usize1; N]
    }
    let mut dp = vec![vec![0; N + 1]; 100];
    for i in 0..N {
        dp[0][i] = A[i];
    }

    for i in 1..100 {
        for j in 0..N {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    let mut ans = 0;
    let mut bidx = 0;
    while K > 0 {
        if K & 1 > 0 {
            ans = dp[bidx][ans];
        }
        K >>= 1;
        bidx += 1;
    }

    println!("{}", ans + 1)
}
