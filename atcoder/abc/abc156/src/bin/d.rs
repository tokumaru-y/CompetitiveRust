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
fn calc(x: usize, N: usize) -> usize {
    let mut res = 1;
    for i in 0..x {
        res *= N - i;
        res %= MOD;
    }
    for i in 1..(x + 1) {
        res *= modinverse(i);
        res %= MOD;
    }
    res
}
fn modpow(multi: usize, num: usize) -> usize {
    let mut res = 1;
    let mut m = multi;
    let mut copy_num = num;
    while copy_num > 0 {
        if (copy_num & 1) == 1 {
            res *= m;
            res %= MOD;
        }
        m *= m;
        m %= MOD;
        copy_num >>= 1;
    }
    return res;
}

fn modinverse(num: usize) -> usize {
    modpow(num, MOD - 2)
}
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
    }
    let mut total = modpow(2, N);
    total -= 1;
    let a_t = calc(A, N);
    let b_t = calc(B, N);
    total += MOD - a_t;
    total %= MOD;
    total += MOD - b_t;
    total %= MOD;
    println!("{}", total);
}
