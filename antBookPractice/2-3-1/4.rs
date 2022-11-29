// https://atcoder.jp/contests/abc015/tasks/abc015_4
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
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
    process::exit,
};
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 10_000_000;
#[allow(non_snake_case)]
fn main() {
    input! {
        W: usize,
        N: usize,
        K: usize,
        AB: [(isize, isize); N]
    }
    let mut dp = vec![vec![-1; W + 1]; K + 1];

    for i in 0..=K {
        dp[i][0] = 0;
    }
    for (a, b) in AB.iter() {
        for i in (0..K).rev() {
            for j in (0..=W).rev() {
                if dp[i][j] == -1 {
                    continue;
                }
                if *a as usize + j <= W {
                    dp[i + 1][*a as usize + j] = max(dp[i + 1][*a as usize + j], dp[i][j] + *b);
                }
                dp[i + 1][j] = max(dp[i + 1][j], dp[i][j]);
            }
        }
    }
    let mut ans = dp[K].iter().max().unwrap();
    println!("{}", ans);
}
