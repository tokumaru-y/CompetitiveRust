// https://atcoder.jp/contests/yahoo-procon2018-qual/tasks/yahoo_procon2018_qual_c
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

fn calc(
    sold_i: usize,
    msk: usize,
    dp: &mut Vec<Vec<isize>>,
    value_dp: &Vec<Vec<usize>>,
    N: usize,
) -> isize {
    if dp[sold_i][msk] >= 0 {
        return dp[sold_i][msk];
    }
    if sold_i == N {
        return 0;
    }

    let mut sold = std::isize::MAX;
    for i in 0..N {
        if msk & (1 << i) > 0 {
            sold = min(sold, calc(sold_i + 1, (msk ^ (1 << i)), dp, value_dp, N))
        }
    }

    let buy = value_dp[sold_i][msk] as isize;

    dp[sold_i][msk] = max(sold, buy);

    dp[sold_i][msk]
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: [usize; N],
        Y: [usize; N],
        Z: [usize; N]
    }
    let mut value_dp = vec![vec![0; (1 << N)]; N + 1];

    for i in 0..=N {
        let mut yen = X[..i].iter().sum::<usize>();

        for msk in 0..(1 << N) {
            let mut cost = 0;
            let mut value = 0;
            for j in 0..N {
                if msk & (1 << j) > 0 {
                    cost += Y[j];
                    value += Z[j];
                }

                if cost <= yen {
                    value_dp[i][msk] = value;
                } else {
                    value_dp[i][msk] = 0;
                }
            }
        }

        for msk in 0..(1 << N) {
            for j in 0..N {
                if msk & (1 << j) > 0 {
                    value_dp[i][msk] = max(value_dp[i][msk], value_dp[i][msk ^ (1 << j)]);
                }
            }
        }
    }

    let mut dp = vec![vec![-1; (1 << N) + 1]; N + 1];

    calc(0, (1 << N) - 1, &mut dp, &value_dp, N);

    println!("{}", dp[0][(1 << N) - 1])
}
