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
use std::{
    fmt::Debug,
    io::{stdout, Write},
    mem::swap,
};
fn unwrap_result_type<T: Debug>(x: Result<T, T>) -> T {
    if x.is_ok() {
        x.unwrap()
    } else {
        x.unwrap_err()
    }
}

fn read() -> usize {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<usize>().unwrap()
}
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = std::usize::MAX;
const MOD: usize = 1_000_000_007;

#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    let mut ans = 0;
    let one_pair_cnt = (B - A + 1) / 2;
    let add = if one_pair_cnt % 2 == 0 { 0 } else { 1 };

    if A % 2 == 0 && B % 2 == 0 {
        ans = B ^ add;
    } else if A % 2 == 0 && B % 2 == 1 {
        ans = add;
    } else if A % 2 == 1 && B % 2 == 0 {
        ans = B ^ A ^ (((B - A - 1) / 2) % 2);
    } else {
        ans = A ^ add;
    }

    println!("{}", ans);
}
