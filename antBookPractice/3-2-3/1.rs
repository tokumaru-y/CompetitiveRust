// https://atcoder.jp/contests/arc064/tasks/arc064_a
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
        X: usize,
        A: [usize; N]
    }

    let mut ans = 0;
    let mut new_A = Vec::new();
    for i in 0..=N {
        if i == 0 {
            new_A.push(0);
        } else {
            new_A.push(A[i - 1]);
        }
    }

    for i in 0..N {
        let mut sum = new_A[i] + new_A[i + 1];
        if sum > X {
            ans += sum - X;
            new_A[i + 1] -= sum - X;
        }
    }

    println!("{}", ans);
}
