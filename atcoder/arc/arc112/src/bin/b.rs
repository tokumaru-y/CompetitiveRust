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
        B: isize,
        C: isize,
    }
    let mut ans = 0;
    let rev_center = B * -1;
    let mut rev_right = rev_center;
    let mut rev_left = rev_center;
    if C > 1 {
        rev_right += (C - 1) / 2;
        rev_left -= (C - 1) / 2;
    }

    let mut left = B;
    let mut right = B;
    if C > 1 {
        left -= C / 2;
    }
    if C > 2 {
        right += (C - 2) / 2;
    }
    ans += (right - left + 1) + (rev_right - rev_left + 1);
    if B > 0 && rev_right >= left {
        ans -= (rev_right - left + 1);
    } else if B < 0 && right >= rev_left {
        ans -= (right - rev_left + 1);
    } else if B == 0 {
        ans = right - left + 1;
    }

    println!("{}", ans);
}
