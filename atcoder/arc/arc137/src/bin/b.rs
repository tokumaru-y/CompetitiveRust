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
        A: [usize; N]
    }
    let mut flip_list = vec![0; N + 1];

    for i in 0..N {
        let t = if A[i] == 1 { -1 } else { 1 };
        flip_list[i + 1] = flip_list[i] + t;
    }

    let mut x = 0;
    let mut y = 0;
    let mut cur = 0;
    let mut max_cnt = 0;
    let mut min_cnt = 0;
    for i in 0..N {
        if A[i] == 0 {
            cur += 1;
        } else {
            cur -= 1;
        }

        x = min(x, cur - max_cnt);
        y = max(y, cur - min_cnt);
        min_cnt = min(min_cnt, cur);
        max_cnt = max(max_cnt, cur);
    }

    println!("{}", y - x + 1);
}
