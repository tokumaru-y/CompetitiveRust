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
    iter::FromIterator,
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
        Q: usize,
    }
    let mut que = VecDeque::new();
    let mut p_que: BinaryHeap<isize> = BinaryHeap::new();

    for _ in 0..Q {
        input! {
            q: usize,
        }
        if q == 1 {
            input! {
                x: isize,
            }
            que.push_back(-x);
        } else if q == 2 {
            if let Some(x) = p_que.pop() {
                println!("{}", x.abs());
            } else if let Some(x) = que.pop_front() {
                println!("{}", x.abs());
            }
        } else {
            p_que.append(&mut BinaryHeap::from_iter(que.into_iter()));
            que = VecDeque::new();
        }
    }
}
