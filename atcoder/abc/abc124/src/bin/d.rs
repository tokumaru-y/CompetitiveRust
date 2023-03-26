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

fn exec(S: Vec<char>) -> Vec<(char, usize)> {
    let mut res = Vec::new();
    let mut pre = S[0];
    let mut cnt = 1;
    for i in 1..(S.len()) {
        if pre == S[i] {
            cnt += 1;
        } else {
            res.push((pre, cnt));
            pre = S[i];
            cnt = 1;
        }
    }
    res.push((pre, cnt));
    res
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        S: String
    }
    let mut run_length = exec(S.chars().collect_vec());
    let M = run_length.len();
    let mut ans = 0;
    let mut sum = 0;
    let mut left = 0;
    let mut zero_cnt = 0;

    for right in 0..M {
        let (c, v) = run_length[right];
        sum += v;
        if c == '0' {
            zero_cnt += 1;
        }

        while left < right && zero_cnt > K {
            let (lc, lv) = run_length[left];
            sum -= lv;
            if lc == '0' {
                zero_cnt -= 1;
            }
            left += 1;
        }

        ans = max(ans, sum);
    }

    println!("{}", ans);
}
