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
        mut P: [usize; N]
    }
    let mut ans = vec![];
    for i in 0..N {
        for j in 0..(N - 2) {
            if P[j] % 2 != P[j + 2] % 2 && P[j] % 2 != j % 2 {
                ans.push(('B', j + 1));
                P.swap(j, j + 2);
            }
        }
    }
    for i in 0..(N - 1) {
        if P[i] % 2 == i % 2 {
            ans.push(('A', i + 1));
            P.swap(i, i + 1);
        }
    }

    for i in 0..N {
        for j in 0..(N - 2) {
            if P[j] > P[j + 2] {
                ans.push(('B', j + 1));
                P.swap(j, j + 2);
            }
        }
    }

    println!("{}", ans.len());
    for (a, b) in ans.into_iter() {
        println!("{} {}", a, b);
    }
}
