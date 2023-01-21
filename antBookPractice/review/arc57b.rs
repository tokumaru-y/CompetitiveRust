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
        K: usize,
        A: [usize; N]
    }
    let mut dp = vec![vec![std::usize::MAX; N + 1]; N + 1];

    let mut total_sum = A.iter().sum::<usize>();
    if K == total_sum {
        println!("1");
        exit(0);
    }

    let mut total_game_cnt = vec![0; N + 1];
    for i in 0..N {
        total_game_cnt[i + 1] = total_game_cnt[i] + A[i];
    }
    dp[1][0] = 0;
    dp[1][1] = 1;
    for i in 1..N {
        for j in 0..=i {
            if dp[i][j] == std::usize::MAX {
                continue;
            }
            let min_cnt =
                dp[i][j] * (total_game_cnt[i + 1] - total_game_cnt[i]) / total_game_cnt[i];
            if min_cnt + 1 <= A[i] {
                dp[i + 1][j + 1] = min(dp[i][j] + min_cnt + 1, dp[i + 1][j + 1]);
            }

            dp[i + 1][j] = min(dp[i + 1][j], dp[i][j]);
        }
    }

    for j in (0..=N).rev() {
        if dp[N][j] <= K {
            println!("{}", j);
            exit(0)
        }
    }
}
