// https://atcoder.jp/contests/tdpc/tasks/tdpc_dice
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
        N: usize,
        mut D: usize,
    }
    if D == 1 {
        println!("1");
        exit(0);
    }
    let mut dp = vec![[[[0.0; 40]; 60]; 110]; N + 1];
    let mut d_num_list = [0, 0, 0];

    for (i, n) in [2, 3, 5].into_iter().enumerate() {
        while D % n == 0 {
            D /= n;
            d_num_list[i] += 1;
        }
    }
    let [a, b, c] = d_num_list;
    if D != 1 {
        println!("0");
        exit(0);
    }

    dp[0][0][0][0] = 1.0;
    for time in 0..N {
        for i in (0..=a) {
            for j in (0..=b) {
                for k in (0..=c) {
                    dp[time + 1][i][j][k] += dp[time][i][j][k] / 6.0;
                    dp[time + 1][min(a, i + 1)][j][k] += dp[time][i][j][k] / 6.0;
                    dp[time + 1][i][min(b, j + 1)][k] += dp[time][i][j][k] / 6.0;
                    dp[time + 1][min(a, i + 2)][j][k] += dp[time][i][j][k] / 6.0;
                    dp[time + 1][i][j][min(c, k + 1)] += dp[time][i][j][k] / 6.0;
                    dp[time + 1][min(a, i + 1)][min(b, j + 1)][k] += dp[time][i][j][k] / 6.0;
                }
            }
        }
    }
    println!("{}", dp[N][a][b][c]);
}
