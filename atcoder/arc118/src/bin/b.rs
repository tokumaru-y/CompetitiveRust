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
        K: usize,
        N: isize,
        M: isize,
        A: [isize; K]
    }

    let mut b = vec![];
    for i in 0..K {
        b.push(M * A[i] / N);
    }

    let mut diff = vec![];
    for i in 0..K {
        diff.push((N * b[i] - M * A[i], i));
    }

    diff.sort();
    let mut left_add = M - b.iter().sum::<isize>();
    for i in 0..left_add {
        b[diff[i as usize].1] += 1;
    }

    println!("{}", b.into_iter().join(" "));
}
