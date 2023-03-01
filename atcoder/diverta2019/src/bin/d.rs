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
fn divisor_list(n: usize) -> Vec<usize> {
    let mut res = Vec::new();
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            if i >= 3 && (n / (i - 1) == i) {
                res.push(i);
            }
            if i != n / i && n / i >= 3 && (n / (n / i - 1) == i) {
                res.push(n / i);
            }
        }
        i += 1;
    }
    res
}
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut div_list = divisor_list(N);
    let len = div_list.len();
    let ans: usize = div_list.iter().sum::<usize>() - len;
    println!("{}", ans);
}
