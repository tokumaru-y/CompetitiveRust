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
const MOD: usize = 998244353;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N]
    }
    A.sort();
    let mut ans = 0;
    let mut d = A[N - 1];
    for i in (0..N).rev() {
        ans += A[i] * d;
        ans %= MOD;
        if i == 0 {
            continue;
        }
        d -= A[i];
        d *= 2;
        d %= MOD;
        d += A[i] + A[i - 1];
    }

    println!("{}", ans % MOD);
}
