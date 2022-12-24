// https://atcoder.jp/contests/code-festival-2015-quala/tasks/codefestival_2015_qualA_d
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

fn is_acceptable(N: usize, X: &Vec<usize>, T: usize) -> bool {
    let mut checked_index = 0;
    for i in 0..X.len() {
        let need_reach_left_ind = checked_index + 1;
        if checked_index < X[i] && X[i] - need_reach_left_ind > T {
            return false;
        }

        if checked_index >= X[i] {
            checked_index = max(checked_index, X[i] + T)
        } else {
            // right -> X[i] -> left
            let a = X[i] + (T - (X[i] - need_reach_left_ind)) / 2;
            // left -> X[i] -> right'
            let b = X[i] + T - (X[i] - need_reach_left_ind) * 2;

            checked_index = max(a, checked_index);
            checked_index = max(b, checked_index);
        }
    }

    checked_index >= N
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        X: [usize; M]
    }
    let mut ng = -1isize;
    let mut ok = 10usize.pow(10);
    while ok as isize - ng > 1 {
        let mid = (ok as isize + ng) / 2;

        if is_acceptable(N, &X, mid as usize) {
            ok = mid as usize;
        } else {
            ng = mid as isize;
        }
    }

    println!("{}", ok);
}
