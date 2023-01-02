// https://atcoder.jp/contests/joi2017yo/tasks/joi2017yo_d
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Isize1, Usize1},
};
use std::fmt::Debug;
#[allow(unused_imports)]
use std::{
    cmp::Reverse,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    process::exit,
};
fn unwrap_result_type<T: Debug>(x: Result<T, T>) -> T {
    if x.is_ok() {
        x.unwrap()
    } else {
        x.unwrap_err()
    }
}
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        goods: [usize; N]
    }
    let mut dp = vec![FIRST_VALUE; (1 << M) + 1];
    dp[0] = 0;
    let mut sum_cnt = vec![0; M];
    let mut cum_sum = vec![vec![0; N + 1]; M];

    for i in 0..N {
        sum_cnt[goods[i] - 1] += 1;
        cum_sum[goods[i] - 1][i] += 1;
    }

    for i in 0..M {
        for j in 0..N {
            cum_sum[i][j + 1] += cum_sum[i][j];
        }
    }

    for bit in 0..(1 << M) {
        let mut done = 0;
        for i in 0..M {
            if (bit & 1 << i) > 0 {
                done += sum_cnt[i];
            }
        }

        for nx in 0..M {
            if (bit & (1 << nx)) == 0 {
                let mut total_cost = dp[bit];

                let mut next_cnt = cum_sum[nx][done + sum_cnt[nx] - 1];
                if done > 0 {
                    next_cnt -= cum_sum[nx][done - 1];
                }
                total_cost += sum_cnt[nx] - next_cnt;

                dp[bit | (1 << nx)] = min(dp[bit | (1 << nx)], total_cost);
            }
        }
    }
    println!("{}", dp[(1 << M) - 1]);
}
