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
        A: [isize; N]
    }
    let mut dp = vec![vec![0isize; 3010]; 3010];

    for len in 1..=N {
        for i in 0..=(N) {
            if i + len > N {
                continue;
            }
            let j = i + len;

            if ((N - len) % 2 == 0) {
                dp[i][j] = max(dp[i + 1][j] + A[i], dp[i][j - 1] + A[j - 1]);
            } else {
                dp[i][j] = min(dp[i + 1][j] - A[i], dp[i][j - 1] - A[j - 1]);
            }
        }
    }

    println!("{}", dp[0][N]);
}
