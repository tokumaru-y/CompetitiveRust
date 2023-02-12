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
        M: usize,
        F: [usize; N]
    }
    let mut dp = vec![0; N + 1];
    let mut sum = vec![0; N + 2];
    let mut left_index = vec![0; N + 1];

    let mut left = 0;
    let mut fnum = vec![0; M + 1];
    for right in 0..N {
        fnum[F[right]] += 1;
        while left < right && fnum[F[right]] > 1 {
            fnum[F[left]] -= 1;
            left += 1;
        }
        left_index[right + 1] = left;
    }

    sum[1] = 1;
    for i in 1..=N {
        dp[i] = (sum[i] - sum[left_index[i]] + MOD) % MOD;
        sum[i + 1] = (sum[i] + dp[i]) % MOD;
    }

    println!("{}", dp[N]);
}
