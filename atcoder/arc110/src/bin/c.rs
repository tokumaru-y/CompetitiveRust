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

fn search(left: usize, ans: &mut Vec<usize>, P: &mut Vec<usize>, P_idx: &mut Vec<usize>) -> bool {
    if ans.len() == P.len() - 1 {
        for i in 0..P.len() {
            if P[i] != i {
                return false;
            }
        }
        return true;
    }

    if P_idx[left] <= left {
        return false;
    }

    for i in ((left + 1)..=P_idx[left]).rev() {
        ans.push(i);
        P.swap(i - 1, i)
    }
    for i in left..P_idx[left] {
        if P[i] != i {
            return false;
        }
    }
    return search(P_idx[left], ans, P, P_idx);
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut P: [Usize1; N]
    }
    let mut P_idx = vec![0; N];
    for i in 0..N {
        P_idx[P[i]] = i;
    }

    let mut ans = vec![];
    if !search(0, &mut ans, &mut P, &mut P_idx) {
        println!("-1")
    } else {
        for a in ans.into_iter() {
            println!("{}", a)
        }
    }
}
