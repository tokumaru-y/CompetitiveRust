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
        mut T: [usize; N]
    }
    let mut dp = vec![vec![false; 10usize.pow(5) + 1]; N + 1];
    let mut ans = std::usize::MAX;

    dp[0][0] = true;
    for i in 0..N {
        for j in (0..=10usize.pow(5)).rev() {
            if dp[i][j] {
                dp[i + 1][j + T[i]] = true;
            }
            dp[i + 1][j] |= dp[i][j];
        }
    }

    let sum = T.iter().sum::<usize>();
    for j in (0..=10usize.pow(5)) {
        if !dp[N][j] {
            continue;
        }
        ans = min(ans, max(j, sum - j));
    }

    println!("{}", ans);
}
