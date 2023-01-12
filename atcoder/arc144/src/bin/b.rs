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
fn is_acceptable(x: usize, X: &Vec<usize>, A: usize, B: usize) -> bool {
    let mut add_cnt = 0;
    let mut dif_cnt = 0;
    for i in 0..(X.len()) {
        if x >= X[i] {
            add_cnt += (x - X[i]) / A;
            if (x - X[i]) % A != 0 {
                add_cnt += 1;
            }
        } else {
            dif_cnt += (X[i] - x) / B;
        }
    }

    dif_cnt >= add_cnt
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
        X: [usize; N]
    }

    let mut is_ok = 0;
    let mut is_ng = 10usize.pow(10);
    while is_ng - is_ok > 1 {
        let mid = (is_ng + is_ok) / 2;
        if is_acceptable(mid, &X, A, B) {
            is_ok = mid;
        } else {
            is_ng = mid;
        }
    }

    println!("{}", is_ok);
}
