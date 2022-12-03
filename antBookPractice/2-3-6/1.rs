// https://atcoder.jp/contests/chokudai_S001/tasks/chokudai_S001_h
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
        A: [usize; N]
    }
    let mut dp = vec![FIRST_VALUE; N + 1];
    for a in A.iter() {
        let b_search = dp.binary_search(a);
        let ind = if b_search.is_ok() {
            b_search.unwrap()
        } else {
            b_search.unwrap_err()
        };
        dp[ind] = *a;
    }
    println!("{}", dp.iter().filter(|x| **x != FIRST_VALUE).count())
}
