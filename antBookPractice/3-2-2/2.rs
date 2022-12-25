// https://atcoder.jp/contests/abc017/tasks/abc017_4
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
        F: [usize; N]
    }
    let mut cnt_table = vec![0; M + 1];
    // 各rightに対する適用可能な最左端のindex
    let mut left_index_list = vec![0; N + 1];

    let mut left = 0;
    for right in (0..N) {
        cnt_table[F[right]] += 1;

        while left < right && cnt_table[F[right]] > 1 {
            cnt_table[F[left]] -= 1;
            left += 1;
        }

        left_index_list[right + 1] = left;
    }

    let mut dp = vec![0; N + 1];
    let mut sum = vec![0; N + 2];

    dp[0] = 1;
    sum[1] = 1;
    for i in (1..=N) {
        dp[i] = (sum[i] - sum[left_index_list[i]] + MOD) % MOD;
        sum[i + 1] = (sum[i] + dp[i]) % MOD;
    }

    println!("{}", dp[N]);
}
