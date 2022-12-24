// https://atcoder.jp/contests/abc023/tasks/abc023_d

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

fn is_acceptable(HS: &Vec<(usize, usize)>, x: isize) -> bool {
    let mut time_v = Vec::new();
    for (h, s) in HS.iter() {
        if *h > x as usize {
            return false;
        }
        let time = (x - *h as isize) / *s as isize;
        time_v.push(time);
    }
    time_v.sort();

    for i in 0..time_v.len() {
        if time_v[i] < i as isize {
            return false;
        }
    }

    true
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        HS: [(usize, usize); N]
    }
    let mut ng = 0;
    let mut ok = 10usize.pow(19);

    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        if is_acceptable(&HS, mid as isize) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
