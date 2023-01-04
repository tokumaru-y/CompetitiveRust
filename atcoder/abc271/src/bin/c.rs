#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Isize1, Usize1},
};
use std::fmt::Debug;
#[allow(unused_imports)]
use std::{
    cmp::Reverse,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    process::exit,
};
fn unwrap_result_type<T: Debug>(x: Result<T, T>) -> T {
    if x.is_ok() {
        x.unwrap()
    } else {
        x.unwrap_err()
    }
}
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }
    let mut sold_cnt = 0;
    let mut is_exists = vec![false; N + 10];

    for i in 0..N {
        if A[i] > N {
            sold_cnt += 1;
        } else if is_exists[A[i]] {
            sold_cnt += 1;
        } else {
            is_exists[A[i]] = true;
        }
    }

    let mut min_cnt = 1;
    let mut max_cnt = N + 1;

    while true {
        while is_exists[min_cnt] {
            min_cnt += 1;
        }
        while max_cnt > 0 && !is_exists[max_cnt] {
            max_cnt -= 1;
        }

        if sold_cnt >= 2 {
            sold_cnt -= 2;
            is_exists[min_cnt] = true;
        } else {
            if min_cnt >= max_cnt {
                break;
            }
            is_exists[max_cnt] = false;
            sold_cnt += 1;
        }
    }

    println!("{}", min_cnt - 1);
}
