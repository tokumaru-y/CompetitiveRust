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
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: Chars
    }
    let mut s_i = FIRST_VALUE;

    for i in 0..N {
        if S[i] == 'd' {
            continue;
        }
        if S[i] == 'p' {
            s_i = i;
            break;
        }
    }

    let mut ans = S.clone();
    if s_i != FIRST_VALUE {
        for k in (s_i + 1)..=N {
            let mut tmp = S[0..s_i].to_vec();
            let t = S[s_i..k].iter().map(|x| if *x == 'p' { 'd' } else { 'p' });
            for a in t.rev().into_iter() {
                tmp.push(a);
            }
            for s in (k..N) {
                tmp.push(S[s]);
            }
            ans = min(ans, tmp);
        }
    }

    println!("{}", ans.iter().join(""));
}
