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
        X: [isize; N],
        CY: [(usize, usize); M]
    }
    let mut dp = vec![vec![-1isize; N + 1]; N + 1];
    let mut bonus_table = vec![0; N + 1];

    for (c, y) in CY.into_iter() {
        bonus_table[c] = y;
    }

    dp[0][0] = 0;
    for i in 0..N {
        for j in 0..=i {
            if dp[i][j] >= 0 {
                dp[i + 1][j + 1] = max(dp[i + 1][j + 1], dp[i][j] + X[i]);
                if bonus_table[j + 1] >= 0 {
                    dp[i + 1][j + 1] += bonus_table[j + 1] as isize;
                }
            }
            dp[i + 1][0] = max(dp[i + 1][0], dp[i][j]);
        }
    }

    println!("{}", dp[N].iter().max().unwrap());
}
