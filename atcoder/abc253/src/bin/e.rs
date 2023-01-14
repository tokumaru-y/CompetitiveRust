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
const MOD: usize = 998244353;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
    }
    let mut dp = vec![vec![0usize; M + 1]; N + 1];
    let mut sum_list = vec![vec![0usize; M + 2]; N + 1];
    for i in 1..=M {
        dp[1][i] = 1;
        sum_list[1][i] = sum_list[1][i - 1] + dp[1][i];
    }
    for i in 1..N {
        for j in 1..=M {
            if K == 0 {
                dp[i + 1][j] = sum_list[i][M];
                dp[i + 1][j] %= MOD;
            } else {
                let right = j + K;
                if j >= K + 1 {
                    let left = j - K;
                    dp[i + 1][j] += sum_list[i][left];
                    dp[i + 1][j] %= MOD;
                }
                if right <= M {
                    dp[i + 1][j] += (sum_list[i][M] + MOD - sum_list[i][right - 1]) % MOD;
                    dp[i + 1][j] %= MOD;
                }
            }
            sum_list[i + 1][j] = (sum_list[i + 1][j - 1] + dp[i + 1][j]) % MOD;
            sum_list[i + 1][j] %= MOD;
        }
    }
    let mut ans = 0;
    for i in 1..=M {
        ans += dp[N][i];
        ans %= MOD;
    }
    println!("{}", ans);
}
