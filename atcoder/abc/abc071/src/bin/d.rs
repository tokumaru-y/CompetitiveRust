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
        N: usize,
        S: Chars,
        T: Chars
    }
    let mut ans = 1;
    let mut x = 0;
    let mut pre = -1;
    while x < N {
        if S[x] == T[x] {
            if pre < 0 {
                ans = 3;
            } else if pre == 0 {
                ans *= 2;
            } else {
                ans *= 1;
            }
            x += 1;
            pre = 0;
        } else {
            if pre < 0 {
                ans = 6;
            } else if pre == 0 {
                ans *= 2;
            } else {
                ans *= 3;
            }
            x += 2;
            pre = 1;
        }
        ans %= MOD;
    }

    println!("{}", ans);
}
