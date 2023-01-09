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
const FIRST_VALUE: isize = std::isize::MIN;
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
        mut X: [usize; K*2]
    }
    let mut ans = vec![];

    for i in (0..=(2 * K - 2)).step_by(2) {
        if i == 2 * K - 2 {
            if i % 2 == 0 {
                if X[i] > X[i + 1] {
                    ans.push(i + 1);
                }
            } else {
                if X[i] < X[i + 1] {
                    ans.push(i + 1);
                }
            }
            break;
        }

        if i % 2 == 0 {
            if X[i] < X[i + 1] && X[i + 1] > X[i + 2] {
                continue;
            }
            if X[i] > X[i + 2] {
                ans.push(i + 1);
                X.swap(i, i + 1);
            } else {
                ans.push(i + 2);
                X.swap(i + 1, i + 2);
            }
        } else {
            if X[i] > X[i + 1] && X[i + 1] < X[i + 2] {
                continue;
            }
            if X[i] < X[i + 2] {
                ans.push(i + 1);
                X.swap(i, i + 1);
            } else {
                ans.push(i + 2);
                X.swap(i + 1, i + 2);
            }
        }
    }
    println!("{}", ans.len());
    if ans.len() > 0 {
        println!("{}", ans.iter().join(" "));
    }
}
