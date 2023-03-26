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
        K: usize,
        P: [usize; N]
    }
    let mut under = vec![0; N + 1];
    let mut pile = vec![0; N + 1];
    let mut btree: BTreeSet<usize> = BTreeSet::new();
    let mut ans = vec![-1; N + 1];

    for i in 0..N {
        let mut range = btree.range(P[i]..);
        let value = if let Some(x) = range.next() {
            Some(*x)
        } else {
            None
        };
        if value.is_some() {
            let x: usize = value.unwrap();
            under[P[i]] = x;
            pile[P[i]] = pile[x] + 1;
            btree.remove(&x);
            btree.insert(P[i]);
        } else {
            pile[P[i]] = 1;
            btree.insert(P[i]);
        }

        if pile[P[i]] == K {
            btree.remove(&P[i]);
            let mut n = P[i];
            for _ in 0..K {
                ans[n] = (i + 1) as isize;
                n = under[n];
            }
        }
    }

    for i in 1..=N {
        println!("{}", ans[i]);
    }
}
