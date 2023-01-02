// https://atcoder.jp/contests/abc180/tasks/abc180_e
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
        XYZ: [(isize, isize, isize); N]
    }
    let mut dp = vec![vec![FIRST_VALUE; N]; 1 << N];
    dp[0][0] = 0;

    for s in 0..(1 << N) {
        for from in 0..N {
            let (a, b, c) = XYZ[from];
            for to in 0..N {
                let (x, y, z) = XYZ[to];
                if dp[s][from] != FIRST_VALUE {
                    dp[s | 1 << from][to] = min(
                        dp[s | 1 << from][to],
                        dp[s][from]
                            + (a - x).abs() as usize
                            + (b - y).abs() as usize
                            + max(0, z - c) as usize,
                    );
                }
            }
        }
    }

    println!("{}", dp[(1 << N) - 1][0]);
}
