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
        mut S: Chars,
    }
    let mut ans = 0;
    if S.len() <= 2 {
        println!("0");
        exit(0);
    }
    let mut tmp_s = vec![];
    let mut i = 0;
    while i <= (S.len() - 1) {
        if i <= (S.len() - 2) && S[i] == 'B' && S[i + 1] == 'C' {
            tmp_s.push('X');
            i += 2;
        } else {
            tmp_s.push(S[i]);
            i += 1;
        }
    }
    let mut a_cnt: usize = 0;
    for i in 0..(tmp_s.len()) {
        if tmp_s[i] == 'A' {
            a_cnt += 1;
        } else if tmp_s[i] == 'X' {
            if a_cnt == 0 {
                continue;
            }

            ans += a_cnt;
        } else {
            a_cnt = 0;
        }
    }

    println!("{}", ans);
}
