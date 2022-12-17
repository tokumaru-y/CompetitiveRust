// https://atcoder.jp/contests/arc017/tasks/arc017_1
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
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    process::exit,
};
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
fn divisor_list(n: &usize) -> Vec<usize> {
    let mut res = Vec::new();
    for i in 1..=((*n as f64).sqrt() as usize) {
        if n % i == 0 {
            res.push(i);
            if i != n / i {
                res.push(n / i);
            }
        }
    }
    res
}
fn main() {
    input! {
        N: usize,
    }
    let num_list = divisor_list(&N);
    let ans = if num_list.len() == 2 { "YES" } else { "NO" };
    println!("{}", ans);
}
