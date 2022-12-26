// https://atcoder.jp/contests/arc088/tasks/arc088_b
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
        S: Chars,
    }
    let len = S.len();

    let mut ans = 10usize.pow(10);
    for i in 0..(len - 1) {
        if S[i] != S[i + 1] {
            let dif = max(i + 1, len - (i + 1));
            ans = min(ans, dif)
        }
    }
    if ans == 10usize.pow(10) {
        ans = len;
    }
    println!("{}", ans);
}
