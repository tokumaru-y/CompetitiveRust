// https://atcoder.jp/contests/arc050/tasks/arc050_b
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

fn is_acceptable(X: usize, Y: usize, R: usize, B: usize, A: usize) -> bool {
    if A > R || A > B {
        return false;
    }
    let m = (R - A) / (X - 1);
    let n = (B - A) / (Y - 1);

    A <= m + n
}

#[allow(non_snake_case)]
fn main() {
    input! {
        R: usize,
        B: usize,
        X: usize,
        Y: usize,
    }

    let mut ok = 0;
    let mut ng = 10usize.pow(19);
    while ng - ok > 1 {
        let mid = (ng + ok) / 2;
        if is_acceptable(X, Y, R, B, mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
