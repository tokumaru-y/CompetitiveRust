#[allow(unused_imports)]
use itertools::Itertools;
use num_integer::Roots;
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
fn make_table(K: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let mut target = K;

    for i in (2..=K.sqrt()) {
        if target % i == 0 {
            let mut cnt = 0;
            while target % i == 0 {
                target /= i;
                cnt += 1;
            }
            res.push((i, cnt));
        }
    }
    if target > 1 {
        res.push((target, 1));
    }

    res
}

fn div_cnt(N: usize, P: usize) -> usize {
    let mut res = 0;
    let mut N = N;
    while N % P == 0 {
        N /= P;
        res += 1;
    }

    res
}
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
    }
    let cnt_table = make_table(K);

    let mut ans = 0;
    for i in 0..cnt_table.len() {
        let (x, y) = cnt_table[i];
        let mut div = 0;
        for n in (1..=y) {
            div += div_cnt(x * n, x);
            if div >= y {
                ans = max(ans, x * n);
                break;
            }
        }
    }

    println!("{}", ans);
}
