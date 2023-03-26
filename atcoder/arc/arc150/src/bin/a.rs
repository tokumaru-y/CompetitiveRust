#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Isize1, Usize1},
};
use std::fmt::Debug;
#[allow(unused_imports)]
use std::{
    cmp::Reverse,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    process::exit,
};
fn unwrap_result_type<T: Debug>(x: Result<T, T>) -> T {
    if x.is_ok() {
        x.unwrap()
    } else {
        x.unwrap_err()
    }
}
const DXY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const FIRST_VALUE: usize = 1_000_000_000;
const MOD: usize = 1_000_000_007;
#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
    }
    for _ in 0..T {
        input! {
            N: usize,
            K: usize,
            S: Chars,
        }
        let mut sumone = vec![0; N + 1];
        for i in 0..N {
            sumone[i + 1] = if S[i] == '1' {
                sumone[i] + 1
            } else {
                sumone[i]
            };
        }
        let total_one = sumone[N];
        let mut is_ok = false;
        let mut is_dup = false;

        let mut one_cnt = 0;
        let mut char_one = 0;
        for i in 0..K {
            if S[i] == '1' || S[i] == '?' {
                one_cnt += 1;
                if S[i] == '1' {
                    char_one += 1;
                }
            }
        }
        is_ok |= (one_cnt == K && char_one == total_one);

        for i in 1..(N - K + 1) {
            one_cnt -= if S[i - 1] != '0' { 1 } else { 0 };
            one_cnt += if S[i + K - 1] != '0' { 1 } else { 0 };
            char_one = sumone[i + K] - sumone[i];

            if one_cnt == K && char_one == total_one {
                if is_ok {
                    is_dup = true;
                    break;
                } else {
                    is_ok = true;
                }
            }
        }

        if !is_ok || is_dup {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
