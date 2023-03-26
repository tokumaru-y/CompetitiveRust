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
        S: Chars,
        K: usize,
    }
    let mut sub_v = vec![];

    for i in 0..(S.len()) {
        let mut tmp = vec![];
        for j in 0..K {
            if i + j >= S.len() {
                continue;
            }
            tmp.push(S[i + j]);
            sub_v.push(tmp.iter().join(""));
        }
    }

    sub_v.sort();
    sub_v.dedup();

    println!("{}", sub_v[K - 1]);
}
