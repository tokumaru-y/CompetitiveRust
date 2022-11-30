// https://atcoder.jp/contests/maximum-cup-2018/tasks/maximum_cup_2018_d
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
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        L: usize,
        X: usize,
        A: [usize; N]
    }
    let mut dp = vec![vec![FIRST_VALUE; M]; N + 1];
    dp[0][0] = 1;
    for i in (0..N).into_iter() {
        for j in (0..M).into_iter() {
            if dp[i][j] == FIRST_VALUE {
                continue;
            }
            let j_ind = (j + A[i]) % M;
            let add = (j + A[i]) / M;
            dp[i + 1][j_ind] = min(dp[i + 1][j_ind], dp[i][j] + add);
            dp[i + 1][j_ind] = min(dp[i + 1][j_ind], dp[i][j_ind]);
            dp[i + 1][j] = min(dp[i + 1][j], dp[i][j]);
        }
    }
    let ans = if dp[N][L] <= X { "Yes" } else { "No" };
    println!("{}", ans);
}
