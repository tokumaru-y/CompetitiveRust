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
const MOD: usize = 998244353;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut D: [usize; N]
    }
    let mut v: Vec<Vec<usize>> = vec![vec![]; N];
    // let mut node_i = vec![];
    for i in 0..N {
        if i == 0 {
            if D[i] != 0 {
                println!("0");
                exit(0);
            }
        } else {
            if D[i] == 0 {
                println!("0");
                exit(0);
            }
        }
        // node_i.push((D[i], i));
        v[D[i]].push(i);
    }
    for i in 1..N {
        if v[i].len() > 0 && v[i - 1].len() == 0 {
            println!("0");
            exit(0);
        }
    }
    let mut ans = 1;
    for i in 1..N {
        if v[i].len() == 0 {
            break;
        }
        for _ in 0..(v[i].len()) {
            ans *= v[i - 1].len();
            ans %= MOD;
        }
    }
    println!("{}", ans);
}
