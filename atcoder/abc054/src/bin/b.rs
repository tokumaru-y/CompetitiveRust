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
        M: usize,
        A: [Chars; N],
        B: [Chars; M]
    }
    for h in 0..(N - M + 1) {
        for w in 0..(N - M + 1) {
            if A[h][w] == B[0][0] {
                let mut is_ok = true;
                for a in 0..M {
                    for b in 0..M {
                        is_ok &= A[h + a][w + b] == B[a][b];
                    }
                }
                if is_ok {
                    println!("Yes");
                    exit(0);
                }
            }
        }
    }

    println!("No");
}
