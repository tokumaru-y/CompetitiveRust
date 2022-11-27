// https://atcoder.jp/contests/tdpc/tasks/tdpc_contest
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
        P: [usize; N]
    }
    let mut max_num = 100 * 100;
    let mut dp = vec![false; max_num + 1];
    dp[0] = true;
    for p_i in 0..N {
        for i in (0..=max_num).rev() {
            if i + P[p_i] <= max_num && dp[i] {
                dp[i + P[p_i]] = true;
            }
        }
    }
    let mut ans = dp.iter().filter(|x| **x == true).count();
    println!("{}", ans);
}
