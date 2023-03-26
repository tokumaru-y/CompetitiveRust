#[allow(unused_imports)]
use itertools::Itertools;
use petgraph::algo::is_cyclic_undirected;
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
        S: Chars,
        T: Chars,
    }
    let mut sc = S.clone();
    let mut tc = T.clone();
    sc.sort();
    tc.sort();
    if sc != tc {
        println!("-1");
        exit(0);
    }

    let mut is_ng = -1;
    let mut is_ok = N as isize + 1;
    while is_ok - is_ng > 1 {
        let middle = (is_ok + is_ng) / 2;
        let target = S[(middle as usize)..].to_vec();
        let mut m_ind = 0;
        for i in 0..N {
            if T[i] == target[m_ind] {
                m_ind += 1
            }

            if m_ind == target.len() as usize {
                break;
            }
        }

        if m_ind == target.len() as usize {
            is_ok = middle;
        } else {
            is_ng = middle;
        }
    }

    println!("{}", is_ok);
}
