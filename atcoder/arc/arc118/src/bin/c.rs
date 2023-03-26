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
    }
    if N == 3 {
        println!("6 15 10");
        exit(0);
    }

    let mut set: BTreeSet<usize> = vec![6, 15, 10].into_iter().collect();
    let max = 10000;
    for i in 1..=max {
        if i * 6 > max {
            break;
        }
        set.insert(i * 6);
    }

    for i in 1..=max {
        if i * 15 > max {
            break;
        }
        set.insert(i * 15);
    }

    for i in 1..=max {
        if i * 10 > max {
            break;
        }
        set.insert(i * 10);
    }
    let mut v: Vec<usize> = set.into_iter().collect();
    let mut ans = vec![];
    for i in 0..N {
        ans.push(v[i]);
    }
    print!("{}", ans.into_iter().join(" "));
}
