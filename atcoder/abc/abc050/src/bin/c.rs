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
// 繰り返し二乗法
fn modpow(aa: usize, nn: u128) -> u128 {
    // a^n mod  fermat
    let mut a = aa as u128;
    let mut n = nn;
    let mod_num = MOD as u128;
    let mut res: u128 = 1;
    while n > 0 {
        if n & 1 == 1 {
            res = res * a % mod_num;
        }
        a = a * a % mod_num;
        n >>= 1;
    }
    res
}
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }
    let mut is_ok = true;
    A.sort();
    if N % 2 == 0 {
        for idx in (0..N).step_by(2) {
            if A[idx] != idx + 1 || A[idx + 1] != idx + 1 {
                is_ok = false;
                break;
            }
        }
    } else {
        let mut idx = 0;
        while idx < N {
            if idx == 0 {
                if A[0] != 0 {
                    is_ok = false;
                    break;
                }
                idx += 1;
            } else {
                if A[idx] != idx + 1 || A[idx + 1] != idx + 1 {
                    is_ok = false;
                    break;
                }
                idx += 2;
            }
        }
    }

    if is_ok {
        println!("{}", modpow(2, N as u128 / 2));
    } else {
        println!("0");
    }
}
