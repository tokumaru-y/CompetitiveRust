// https://atcoder.jp/contests/abc034/tasks/abc034_d
use alga::general::Lattice;
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

fn is_acceptable(K: usize, WP: &Vec<(usize, usize)>, M: f64) -> bool {
    let mut solt_v = Vec::new();
    for i in (0..WP.len()) {
        let (w, p) = WP[i];
        let dif = (p as f64 - M) * w as f64;
        solt_v.push(dif);
    }

    solt_v.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let sum = solt_v[..K].iter().fold(0f64, |sum, x| sum + x);

    sum >= 0.0
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        WP: [(usize, usize); N]
    }
    let mut ok: f64 = 0.0;
    let mut ng = 100.0;

    while ng - ok > 1E-10 {
        let mid = (ok + ng) / 2.0;

        if is_acceptable(K, &WP, mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
