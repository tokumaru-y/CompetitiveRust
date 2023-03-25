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

const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = std::usize::MAX;
const MOD: usize = 1_000_000_007;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: Chars,
        K: usize,
        D: [usize; K]
    }
    let mut ans = vec![];
    let mut can_use = vec![true; 10];
    for d in D.into_iter() {
        can_use[d] = false;
    }
    let mut n = vec![];
    for i in N.into_iter() {
        n.push(i.to_digit(10).unwrap() as usize);
    }
    let mut add_digit = false;
    for i in 0..(n.len()) {
        let mut is_ok = false;
        for j in 0..10 {
            if can_use[j] && n[i] <= j {
                is_ok = true;
            }
        }
        if !is_ok {
            add_digit = true;
            break;
        }
    }
    if add_digit {
        for j in 1..10 {
            if can_use[j] {
                ans.push(j);
                break;
            }
        }
        for j in 0..10 {
            if can_use[j] {
                for _ in 0..(n.len()) {
                    ans.push(j);
                }
                break;
            }
        }

        println!("{}", ans.into_iter().join(""));
        exit(0);
    }

    let l = n.len();
    let mut is_upper = false;
    for i in 0..l {
        for j in 0..10 {
            if is_upper {
                if can_use[j] {
                    ans.push(j);
                    break;
                }
            } else {
                if can_use[j] && j >= n[i] {
                    is_upper |= j > n[i];
                    ans.push(j);
                    break;
                }
            }
        }
    }

    println!("{}", ans.into_iter().join(""));
}
