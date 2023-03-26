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
        H: usize,
        W: usize,
        S: [Chars; H]
    }
    let mut ans = 1;
    let mut red_table = vec![0; 500 * 500 + 1];
    let mut blue_table = vec![0; 500 * 500 + 1];
    let mut any_table = vec![0; 500 * 500 + 1];

    for i in 0..H {
        for j in 0..W {
            if S[i][j] == 'R' {
                red_table[i + j] += 1;
            } else if S[i][j] == 'B' {
                blue_table[i + j] += 1;
            } else {
                any_table[i + j] += 1;
            }
        }
    }

    for i in 0..(500 * 500 + 1) {
        if red_table[i] > 0 && blue_table[i] > 0 {
            ans = 0;
            break;
        }
        if red_table[i] + blue_table[i] == 0 && any_table[i] > 0 {
            ans *= 2;
        }
        ans %= MOD;
    }

    println!("{}", ans);
}
