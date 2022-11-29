// https://atcoder.jp/contests/tdpc/tasks/tdpc_number
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
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
fn main() {
    input! {
        D: usize,
        mut N: Chars
    }
    let len = N.len();
    let mut dp = vec![vec![vec![0; 2]; D]; len + 1];
    dp[0][0][0] = 1;

    for i in 0..len {
        let limit_num = N[i].to_digit(10).unwrap() as usize;
        for j in 0..D {
            // N未満からN未満に遷移
            for k in 0..=9 {
                dp[i + 1][(j + k) % D][1] += dp[i][j][1];
                dp[i + 1][(j + k) % D][1] %= MOD;
            }
            // N以上からN未満に遷移
            for k in 0..limit_num {
                dp[i + 1][(j + k) % D][1] += dp[i][j][0];
                dp[i + 1][(j + k) % D][1] %= MOD;
            }
            // N以上からN以上へ
            dp[i + 1][(j + limit_num) % D][0] = dp[i][j][0];
            dp[i + 1][(j + limit_num) % D][0] %= MOD;
        }
    }
    let ans = dp[len][0][0] + dp[len][0][1] - 1;
    println!("{}", ans % MOD);
}
