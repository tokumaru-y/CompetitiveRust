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

fn b_search(x: usize, A: &Vec<usize>, B: &Vec<usize>, K: usize) -> bool {
    let mut cnt = 0;
    for i in 0..B.len() {
        let mut target = (x / A[i]);
        let ind = upper_bound(&B, target);
        cnt += ind;
    }

    cnt >= K as usize
}

// xより大きい要素の一番左のindexを返す。
fn upper_bound<T: std::cmp::PartialOrd>(vec: &Vec<T>, x: T) -> usize {
    let mut is_ng: isize = -1;
    let mut is_ok: isize = vec.len() as isize;
    while (is_ok - is_ng) > 1 {
        let mid = (is_ok + is_ng) / 2;

        if x < vec[mid as usize] {
            is_ok = mid;
        } else {
            is_ng = mid;
        }
    }

    is_ok as usize
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N],
        mut B: [usize; N]
    }

    B.sort();

    let mut left = 0;
    let mut right = 10usize.pow(19);
    while (right - left) > 1 {
        let middle = (left + right) / 2;
        if b_search(middle, &A, &B, K) {
            right = middle;
        } else {
            left = middle;
        }
    }
    println!("{}", right);
}
