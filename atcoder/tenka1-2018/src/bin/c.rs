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
        N: usize,
        mut A: [usize; N]
    }
    A.sort();
    A.reverse();

    if N % 2 == 0 {
        let mut res = 0;
        for i in 0..(N / 2 - 1) {
            res += A[i] * 2;
        }
        res += A[N / 2 - 1];
        res -= A[N / 2];
        for i in (N / 2 + 1)..N {
            res -= A[i] * 2;
        }
        println!("{}", res);
    } else {
        let mut res1 = 0;
        for i in 0..(N / 2 - 1) {
            res1 += A[i] * 2;
        }
        res1 += A[N / 2 - 1] + A[N / 2];
        for i in (N / 2 + 1)..N {
            res1 -= A[i] * 2;
        }

        let mut res2 = 0;
        for i in 0..(N / 2) {
            res2 += A[i] * 2;
        }
        res2 -= A[N / 2] + A[N / 2 + 1];
        for i in (N / 2 + 2)..N {
            res2 -= A[i] * 2;
        }
        let ans = max(res1, res2);
        println!("{}", ans);
    }
}
