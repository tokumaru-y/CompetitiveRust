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
        A: [usize; N]
    }
    let mut dp = vec![vec![0usize; N + 1]; 61];

    for i in 0..N {
        dp[0][i] = A[i];
    }

    for i in 0..41 {
        for j in 0..N {
            dp[i + 1][j] = dp[i][j] + dp[i][(j + dp[i][j]) % N];
        }
    }

    let mut ans = 0;
    let mut idx = 0;
    while K > 0 {
        if K & 1 > 0 {
            ans += dp[idx][ans % N];
        }

        K >>= 1;
        idx += 1;
    }

    println!("{}", ans);
}
