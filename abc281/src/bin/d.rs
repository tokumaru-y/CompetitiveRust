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
        K: usize,
        D: usize,
        A: [usize; N]
    }
    let mut dp = vec![vec![-1; D]; K + 1];
    dp[0][0] = 0;
    for i in (0..N) {
        for j in (0..K).rev() {
            for k in (0..D) {
                if dp[j][k] >= 0 {
                    dp[j + 1][(k + A[i]) % D] =
                        max(dp[j + 1][(k + A[i]) % D], dp[j][k] + A[i] as isize);
                }
            }
        }
    }

    println!("{}", dp[K][0]);
}
