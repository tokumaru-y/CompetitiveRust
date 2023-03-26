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
        T: usize,
    }
    let atcoder = ['a', 't', 'c', 'o', 'd', 'e', 'r'];
    for i in 0..T {
        input! {
            mut s: Chars
        }
        let mut ans = std::isize::MAX;
        let mut is_ok = true;
        let mut is_same = true;
        for j in 0..min(s.len(), atcoder.len()) {
            is_ok &= (s[j] >= atcoder[j]);
            is_same &= (s[j] == atcoder[j]);
        }
        if is_ok && !is_same {
            println!("0");
            continue;
        }
        if is_same && s.len() > 7 {
            println!("0");
            continue;
        }
        for j in 0..s.len() {
            if s[j] != 'a' {
                if j == 0 {
                    ans = 0;
                    break;
                }
                if s[j] <= 't' {
                    ans = j as isize;
                } else {
                    ans = j as isize - 1;
                }
                break;
            }
        }
        if ans < std::isize::MAX {
            println!("{}", ans);
        } else {
            println!("-1");
        }
    }
}
