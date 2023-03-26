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
        M: usize,
        A: [isize; N]
    }

    let mut dp = vec![vec![FIRST_VALUE; M + 1]; N + 1];
    dp[0][0] = 0;
    for i in 0..N {
        for j in (0..M).rev() {
            if dp[i][j] != FIRST_VALUE {
                dp[i + 1][j + 1] = max(dp[i + 1][j + 1], dp[i][j] + (A[i] * (j as isize + 1)));
            }
            dp[i + 1][j + 1] = max(dp[i + 1][j + 1], dp[i][j + 1]);
            dp[i + 1][j] = max(dp[i + 1][j], dp[i][j]);
        }
    }

    let ans = dp[N][M];
    println!("{}", ans);
}
