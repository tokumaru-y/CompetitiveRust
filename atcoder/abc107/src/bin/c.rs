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
        N: usize,
        K: usize,
        mut A: [isize; N]
    }
    let mut ans = std::isize::MAX;
    A.sort();
    let mut minus_vec = vec![];
    let mut plus_vec = vec![];
    for i in 0..N {
        if A[i] < 0 {
            minus_vec.push(A[i]);
        } else {
            plus_vec.push(A[i]);
        }
    }
    minus_vec.reverse();
    let mut sum_m = vec![0isize; minus_vec.len() + 1];
    let mut sum_p = vec![0isize; plus_vec.len() + 1];
    for i in 0..(minus_vec.len()) {
        sum_m[i + 1] = minus_vec[i].abs();
    }
    for i in 0..(plus_vec.len()) {
        sum_p[i + 1] = plus_vec[i];
    }

    if N == K {
        let a = sum_m[sum_m.len() - 1];
        let b = sum_p[sum_p.len() - 1];
        ans = if a < b { a * 2 + b } else { a + b * 2 };
        println!("{}", ans);
        exit(0);
    }

    for i in 0..=(min(K, plus_vec.len())) {
        let mut dist: isize = std::isize::MAX;
        if i == 0 {
            if minus_vec.len() < K {
                continue;
            }
            dist = sum_m[K];
        } else if i == K {
            dist = sum_p[K];
        } else {
            if minus_vec.len() < K - i {
                continue;
            }
            let a = sum_p[i];
            let b = sum_m[K - i];
            dist = if a < b { a * 2 + b } else { a + b * 2 };
        }
        ans = min(ans, dist)
    }

    println!("{}", ans);
}
