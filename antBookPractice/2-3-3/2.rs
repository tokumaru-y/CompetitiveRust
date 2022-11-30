// https://atcoder.jp/contests/abc153/tasks/abc153_e
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
        H: usize,
        N: usize,
        AB: [(usize, usize); N]
    }
    let mut dp = vec![FIRST_VALUE; H + 1];
    dp[0] = 0;
    for (a, b) in AB.iter() {
        for i in 0..H {
            if dp[i] == FIRST_VALUE {
                continue;
            }
            dp[min(a + i, H)] = min(dp[min(a + i, H)], dp[i] + b)
        }
    }
    println!("{}", dp[H]);
}
