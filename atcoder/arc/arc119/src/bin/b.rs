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
        mut S: Chars,
        T: Chars
    }

    let mut A = vec![];
    let mut B = vec![];
    for i in 0..N {
        if S[i] == '0' {
            A.push(i);
        }
        if T[i] == '0' {
            B.push(i);
        }
    }

    if A.len() != B.len() {
        println!("-1");
        exit(0);
    }

    let mut ans = 0;
    for i in 0..(A.len()) {
        if A[i] != B[i] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
