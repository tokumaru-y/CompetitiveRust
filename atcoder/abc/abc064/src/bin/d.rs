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
        mut N: usize,
        S: Chars,
    }
    let mut S = VecDeque::from(S);
    let mut cnt: isize = 0;
    let mut pre_cnt: isize = 0;
    for i in 0..N {
        if S[i] == '(' {
            cnt += 1;
        } else {
            cnt -= 1;
            if cnt < 0 {
                pre_cnt = max(pre_cnt, cnt * -1);
            }
        }
    }

    for i in 0..pre_cnt {
        S.push_front('(');
    }
    N += pre_cnt as usize;

    cnt = 0;
    for i in 0..N {
        if S[i] == '(' {
            cnt += 1;
        } else {
            cnt -= 1;
        }
    }

    for i in 0..cnt {
        S.push_back(')');
    }

    println!("{}", S.into_iter().join(""));
}
