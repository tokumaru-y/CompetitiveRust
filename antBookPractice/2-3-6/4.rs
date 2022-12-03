// https://atcoder.jp/contests/tdpc/tasks/tdpc_target
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
        N: usize,
        XR: [(isize, isize); N]
    }
    let mut circles = Vec::new();
    let mut dp = vec![FIRST_VALUE; N];

    for (x, r) in XR.into_iter() {
        let left = x - r;
        let right = x + r;
        circles.push((left, right));
    }
    circles.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));

    for (left, right) in circles.into_iter() {
        let b_search = dp.binary_search(&(right as usize));
        let ind = if b_search.is_ok() {
            b_search.unwrap()
        } else {
            b_search.unwrap_err()
        };
        dp[ind] = right as usize;
    }
    println!("{}", dp.into_iter().filter(|a| *a != FIRST_VALUE).count());
}
