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
fn gcd(x: usize, y: usize) -> usize {
    let mut x = x;
    let mut y = y;
    if x < y {
        std::mem::swap(&mut x, &mut y);
    }
    if y == 0 {
        return x;
    }
    return gcd(y, x % y);
}
#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
        NDK: [(usize, usize, Usize1); T]
    }

    for (n, d, k) in NDK.into_iter() {
        let a = n / gcd(n, d);
        println!("{}", d * k % n + k / a);
    }
}
