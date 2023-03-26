use proconio::input;

fn main() {
    todo!();
}
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

const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = std::usize::MAX;
const MOD: usize = 1_000_000_007;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: isize,
        mut A: [isize; N]
    }

    for i in 0..N {
        A[i as usize] -= i + 1;
    }
    A.sort();
    let d = if N % 2 == 1 {
        A[N as usize / 2]
    } else {
        (A[(N / 2 - 1) as usize] + A[N as usize / 2]) / 2
    };

    let mut ans = 0;
    for i in 0..N {
        ans += (A[i as usize] - d).abs();
    }
    println!("{}", ans);
}
