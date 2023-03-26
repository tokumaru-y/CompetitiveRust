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

fn run_length_encode(S: Vec<char>) -> Vec<(char, usize)> {
    let mut res = vec![];
    let mut dedup_vec = S.clone();
    dedup_vec.dedup();
    let mut j = 0;
    for i in 0..(dedup_vec.len()) {
        let mut cnt = 0;
        while j < S.len() && dedup_vec[i] == S[j] {
            cnt += 1;
            j += 1
        }
        res.push((dedup_vec[i], cnt));
    }

    res
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        S: Chars
    }
    let mut ans = 0;
    let rl = run_length_encode(S);
    let mut group_cnt = rl.len();

    for i in 0..K {
        if 3 <= group_cnt {
            group_cnt -= 2;
        } else if group_cnt == 2 {
            group_cnt = 1;
        }
    }

    ans = N - group_cnt;
    println!("{}", ans);
}
