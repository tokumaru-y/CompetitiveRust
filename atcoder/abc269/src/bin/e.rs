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
    let N = read();
    let mut w_left_ind = 1;
    let mut w_right_ind = N;
    let mut h_high_ind = 1;
    let mut h_low_ind = N;

    while w_left_ind != w_right_ind {
        let mid = (w_left_ind + w_right_ind) / 2;
        println!("? {} {} {} {}", 1, N, w_left_ind, mid);

        let T = read();
        let exp_cnt = mid - w_left_ind + 1;
        if exp_cnt == T {
            w_left_ind = mid + 1;
        } else {
            w_right_ind = mid;
        }
    }
    while h_high_ind != h_low_ind {
        let mid = (h_high_ind + h_low_ind) / 2;
        println!("? {} {} {} {}", h_high_ind, mid, 1, N);
        let T = read();
        let exp_cnt = mid - h_high_ind + 1;
        if exp_cnt == T {
            h_high_ind = mid + 1;
        } else {
            h_low_ind = mid;
        }
    }

    println!("! {} {}", h_high_ind, w_right_ind);
}
